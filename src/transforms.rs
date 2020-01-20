struct ChainEntry(SignedHeader, EntryContent);

struct Notification {
    header: SignedHeader,
    data: NotificationData,
}

enum NotificationData {
    #[cfg(feature = "strict")]
    StoreHeader {
        entry: EntryContent,
    },
    #[cfg(not(feature = "strict"))]
    StoreHeader,
    StoreEntry {
        entry: EntryContent,
    },
    RegisterAgentActivity,
    RegisterUpdatedTo {
        entry: EntryContent,
    },
    RegisterDeletedBy {
        entry: EntryContent,
    },
    RegisterAddLink {
        entry: EntryContent,
    },
    RegisterRemoveLink {
        entry: EntryContent,
    },
}

fn hash_hood(notification: Notification) -> Address {
    use NotificationData::*;

    let Notification { header, data } = notification;
    match data {
        StoreHeader { .. } => hash(header),
        StoreEntry { entry } => header.entry_hash(),
        RegisterAgentActivity => header.author().key_hash(),
        RegisterUpdatedTo { .. } => header.replaces(),
        RegisterDeletedBy { entry } => entry.deletes(),
        RegisterAddLink { entry } => entry.base(),
        RegisterRemoveLink { entry } => entry.base(),
    }
}

// FIXME: avoid allocation or somehow give caller the choice
fn produce_notifications_from_commit(commit: ChainEntry) -> Vec<Notification> {
    let ChainEntry(header, entry) = commit;

    let store_header = Notification {
        // FIXME: avoid cloning
        header: header.clone(),
        #[cfg(feature = "strict")]
        data: NotificationData::StoreHeader {
            // FIXME: avoid cloning
            entry: entry.clone(),
        },
        #[cfg(not(feature = "strict"))]
        data: NotificationData::StoreHeader,
    };
    let store_entry = Notification {
        // FIXME: avoid cloning
        header: header.clone(),
        data: NotificationData::StoreEntry {
            // FIXME: avoid cloning
            entry: entry.clone(),
        },
    };
    let register_agent_activity = Notification {
        // FIXME: avoid cloning
        header: header.clone(),
        data: NotificationData::RegisterAgentActivity,
    };

    let fourth_notification = match header.kind() {
        Create => {
            return vec![store_header, store_entry, register_agent_activity];
        }
        Update => {
            let register_updated_to = Notification {
                // FIXME: avoid cloning
                header: header.clone(),
                data: NotificationData::RegisterUpdatedTo {
                    entry: entry.clone(),
                },
            };
            register_updated_to
        }
        Delete => {
            let register_deleted_by = Notification {
                header: header.clone(),
                data: NotificationData::RegisterDeletedBy {
                    entry: entry.clone(),
                },
            };
            register_deleted_by
        }
        AddLink => {
            let register_add_link = Notification {
                header: header.clone(),
                data: NotificationData::RegisterAddLink {
                    entry: entry.clone(),
                },
            };
            register_add_link
        }
        RemoveLink => {
            let register_remove_link = Notification {
                header: header.clone(),
                data: NotificationData::RegisterRemoveLink {
                    entry: entry.clone(),
                },
            };
            register_remove_link
        }
    };

    vec![
        store_header,
        store_entry,
        register_agent_activity,
        fourth_notification,
    ]
}

#[derive(Hash)]
enum HashReadyForm<'a> {
    // optimization: don't hash signature. it is redundant with header and therefore wastes hash time to include
    StoredHeader {
        header: &'a HeaderWithoutSig,
    },
    StoredEntry {
        header: &'a HeaderWithoutSig,
    },
    RegisteredAgentActivity {
        header: &'a HeaderWithoutSig,
    },
    RegisteredUpdatedTo {
        entry: &'a EntryContent,
        replaces: &'a Address,
    },
    RegisteredDeletedBy {
        entry: &'a DeleteEntry,
    },
    RegisteredAddLink {
        header: &'a HeaderWithoutSig,
    },
    // ^ future work: encode idempotency in LinkAdd entries themselves
    RegisteredRemoveLink {
        header: &'a HeaderWithoutSig,
    },
    // ^ if LinkAdds were idempotent then this could just be entry.
}

fn unique_hash(notification: &Notification) -> HashReadyForm<'_> {
    use NotificationData::*;

    let Notification { header, data } = notification;
    match data {
        StoreHeader { .. } => HashReadyForm::StoredHeader { header },
        StoreEntry { .. } => HashReadyForm::StoredEntry { header },
        RegisterAgentActivity => HashReadyForm::RegisteredAgentActivity { header },
        RegisterUpdatedTo { entry } => HashReadyForm::RegisteredUpdatedTo {
            entry,
            replaces: header.replaces(),
        },
        RegisterDeletedBy { entry } => HashReadyForm::RegisteredDeletedBy { entry },
        RegisterAddLink { .. } => HashReadyForm::RegisteredAddLink { header },
        RegisterRemoveLink { .. } => HashReadyForm::RegisteredRemoveLink { header },
    }
}
