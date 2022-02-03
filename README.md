
# Forte

Bevy ECS ability framework

## Design & Goals
- Attributes are all 64 bit fixed integers with 4 decimal places,
  floating point was avoided for a couple of reasons like adding/removing from an attribute might cause
  issues, however this might not be the case.
- Attributes are also ideally composable. To set up a regenerating effect we shouldn't need to re-implement that for every
  attribute, so instead we have a single `Regen<A>` component for each attribute.
  
  These can also end up being recursive, for example if we wanted a maximum of health *and* a maximum to the max health we would have 3 components, `Attribute<Health>`, `Attribute<Max<Health>>`, and `Attribute<Max<Max<Health>>>`.

- `Ability`s are each their own `Entity`. When an ability hits an interactable it may apply an `Effect` which would be a child entity
  of the interactable or the ability itself.
- `Effect`s are ideally self-reducing to the parent, the developer ultimately should not care what is in the children of the interactable
  entity, only what is on the entity itself. Therefore we need some sort of "registry" stack for effects so we can determine how
  we want to stack effects ontop of eachother.

  For a very simple effect, say a "burn", each child entity would just tick each time it wants to damage the target,
  and just damage an `Attribute`.

  An example of a reducing stack would be a stun, ideally we could query for just a single `Stunned` component
  on the interactable. However, this has issues if they are hit with multiple stuns. So we need a
  kind of "crowdsourced" stun count from each of the effects so that it only removes the `Stunned`
  component when all the stuns have ended.

  In more complex examples, stacking could be done 
