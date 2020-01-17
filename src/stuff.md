## Predicates
- What do they contain?
- If they don't contain EntryWithHeaders, what does?

## Publish Validation Receipts
- How to handle post-commit step of collection validation receipts?
  - Is it considered invalid behaviour by the network to add atop your chain without having acquired enough validation receipts of the old head?

## Transform communication optimiziation/simplification
- Michael's idea:
  - Communicate transforms as BitSet + Invoking Entry
    - BitSet contains 4 bools:
      - Am I expected to hold the header?
      - Am I expected to hold the entry?
      - Am I expected to update the base entry?
      - Am I expected to update the agent's key entry?
- Timo's concern:
  - How does this play with hashing of requests for idempotence?
    - My guess -- not well

## Entry type information -- where is it stored?
- Two layers of type information of source chain elements:
  - Action
    - Create
    - Update
    - Delete
    - AddLink
    - RemoveLink
  - Data type
    - Genesis
    - Agent join
    - App-defined
    - etc.
- Timo is assuming the following. Is it correct?
  - The information about which action type it is goes in the "context"/"metadata"/"header"
  - The information about which data type it is goes in the "content" if relevant


## For UpdateEntry replaces field
- When an agent commits an 'UpdateEntry', I'm assuming that the "replaces" field can't go into the context, because then it would change the hash to be different then if you had just created the entry normally.
  - Does it go in the metadata then?
    - If so, what metadata do chain headers store? I thought it was very minimal.

## Predicate contents

- +h
  - Predicate: None
- +e
  - Predicate: None
- +E -- Create
  - Predicate: None
- +E -- Update
  - Predicate: {Replaces: ()}
- ΔK<sub>a</sub>
  - Predicate: {UpdateAgentActivity: ()}
- ΔE' -- Update
  - Predicate: {UpdateReplaced-by: ()}
- ΔE' -- Delete
  - Predicate: {UpdateDeleted-by: ()}
- ΔE -- AddLink & RemoveLink
  - Predicate: {UpdateLinks: ()}