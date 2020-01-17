struct ChainEntry(EntryMetas, EntryContent);

async fn handle_new_publish(entry: ChainEntry) {
    let (context, content) = entry;
    // publish the "little h" header for the new entry
    let create_little_h = DhtTransform {
        base_hash: hash(context.header()),
        header_hash: panic!(),
        notification: NotificationKind::CreateHeader,
    };
    // modify the author's special DHT entry to point
    // to the newly created "little h" header
    let meta_create_agent_key = DhtTransform {
        base_hash: context.author().key_hash(),
        header_hash: panic!(),
        notification: NotificationKind::AgentActivity,
    };

    match context.kind() {
        CreateEntry => {
            // publish the new "big E" entry itself
            let create_big_e = DhtTransform {
                base_hash: hash(content),
                header_hash: panic!(),
                notification: NotificationKind::CreateEntry,
            };
        }
        UpdateEntry => {
            // publish the creation of the new "big E" entry
            //
            // publish the new "big E" entry itself along with a "replaces" pointer
            // to the old "big E prime" entry
            let create_big_e = DhtTransform {
                base_hash: hash(content),
                header_hash: panic!(),
                notification: NotificationKind::CreateRevision,
            };
            // publish the replacement of the old "big E prime" entry
            //
            // modify the old "big E prime" entry to have a "replaced-by" pointer to
            // the new "big E" entry
            let modify_big_e_prime = DhtTransform {
                // FIXME: Negate need for Option::unwrap
                base_hash: context.as_new_version().unwrap().replaces_hash(),
                header_hash: panic!(),
                notification: NotificationKind::ReplacedBy,
            };
        }
        DeleteEntry => {
            // publish the "little e" entry that invoked the deletion
            // (for reference and proof of agency)
            let create_little_e = DhtTransform {
                base_hash: hash(content),
                header_hash: panic!(),
                notification: NotificationKind::CreateEntry,
            };
            // publish the deletion of the old "big E prime" entry
            //
            // modify the old "big E prime" entry to have a "deleted-by" pointer
            // to the "little e deletion entry" that was just published
            let modify_big_e_prime = DhtTransform {
                // FIXME: Negate need for Option::unwrap
                base_hash: content.as_deletion_proof().unwrap().deletes_hash(),
                header_hash: panic!(),
                notification: NotificationKind::DeletedBy,
            };
        }
        LinkAdd => {
            // publish the "little e" entry that invoked the link addition
            // (for reference and proof of agency)
            let create_little_e = DhtTransform {
                base_hash: hash(content),
                header_hash: panic!(),
                notification: NotificationKind::CreateEntry,
            };
            // publish the adding of the link onto the base
            //
            // modify the base to have a pointer to the target
            // and a pointer the proof (stored in the "little e" entry above)
            let modify_big_e = DhtTransform {
                // FIXME: Negate need for Option::unwrap
                base_hash: content.as_linkadd_proof().unwrap().base_hash(),
                header_hash: panic!(),
                notification: NotificationKind::LinkActivity,
            };
        }
        LinkRemove => {
            // publish the "little e" entry that invoked the link removal
            // (for reference and proof of agency)
            let create_little_e = DhtTransform {
                base_hash: hash(content),
                header_hash: panic!(),
                notification: NotificationKind::CreateEntry,
            };
            // publish the removal of the link from the base
            //
            // modify the base to tombstone the pointer to the target
            // and to have a pointer the proof (stored in the "little e" entry above)
            let modify_big_e = DhtTransform {
                // FIXME: Negate need for Option::unwrap
                base_hash: content.as_linkremove_proof().unwrap().base_hash(),
                header_hash: panic!(),
                notification: NotificationKind::LinkActivity,
            };
        }
    }
}

struct DhtTransform {
    base_hash: Address,
    header_hash: Address,
    notification: NotificationKind,
}

enum NotificationKind {
    CreateHeader, // +h
    // ^ header_hash doesn't really make sense
    CreateEntry, // +e || +E
    // ^ header_hash makes sense
    // TODO: merge CreateRevision with CreateEntry,
    // whether it is a revision or not is already encoded in the header.
    CreateRevision, // +E + --> E'
    // ^ header_hash makes sense
    AgentActivity, // ΔKa + --> h
    // ^ header_hash maybe makes sense (not sure)
    ReplacedBy, // ΔE' + --> E
    // ^
    DeletedBy,    // ΔE' + --> e
    LinkActivity, // ΔE' (base) + --> target live
}
