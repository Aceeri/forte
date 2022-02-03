use bevy::prelude::*;
use fixed::{types::extra::U4, FixedI64};

use std::marker::PhantomData;

pub type Amount = FixedI64<U4>;

#[derive(Component, Debug, Clone)]
pub struct Attribute<A> {
    amount: Amount,
    inner: A,
}

impl<A> Attribute<A>
where
    A: Default,
{
    pub fn new(amount: Amount) -> Self {
        Self {
            amount: amount,
            inner: A::default(),
        }
    }
}

impl<A> Attribute<A> {
    pub fn with_inner(amount: Amount, inner: A) -> Self {
        Self {
            amount: amount,
            inner: inner,
        }
    }

    pub fn inner(&self) -> &A {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut A {
        &mut self.inner
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn set_amount(&mut self, amount: Amount) {
        self.amount = amount;
    }
}

pub struct Base<A>(PhantomData<A>);

pub struct Add<A>(PhantomData<A>);

pub struct Mult<A>(PhantomData<A>);

pub struct Max<A>(PhantomData<A>);

pub struct Min<A>(PhantomData<A>);

pub struct Sub<A>(PhantomData<A>);

fn basic_modifiers<A>(
    mut attributes: Query<
        (
            &mut Attribute<A>,
            Option<&Attribute<Base<A>>>,
            Option<&Attribute<Add<A>>>,
            Option<&Attribute<Mult<A>>>,
        ),
        Or<(
            Changed<Attribute<Base<A>>>,
            Changed<Attribute<Add<A>>>,
            Changed<Attribute<Mult<A>>>,
        )>,
    >,
) where
    A: 'static + Send + Sync,
{
    for (mut current, base, add, mult) in attributes.iter_mut() {
        let base = base.map(|base| base.amount()).unwrap_or(&Amount::ZERO);
        let mult = mult.map(|mult| mult.amount()).unwrap_or(&Amount::ONE);
        let add = add.map(|add| add.amount()).unwrap_or(&Amount::ZERO);
        let result = (base * mult) + add;

        if *current.amount() != result {
            current.set_amount(result);
        }
    }
}

fn clamp_max<A>(
    mut attributes: Query<
        (&mut Attribute<A>, &Attribute<Max<A>>),
        Or<(Changed<Attribute<A>>, Changed<Attribute<Max<A>>>)>,
    >,
) where
    A: 'static + Send + Sync,
{
    for (mut current, max) in attributes.iter_mut() {
        if current.amount() > max.amount() {
            current.set_amount(*max.amount());
        }
    }
}

fn clamp_min<A>(
    mut attributes: Query<
        (&mut Attribute<A>, &Attribute<Min<A>>),
        Or<(Changed<Attribute<A>>, Changed<Attribute<Min<A>>>)>,
    >,
) where
    A: 'static + Send + Sync,
{
    for (mut current, min) in attributes.iter_mut() {
        if current.amount() < min.amount() {
            current.set_amount(*min.amount());
        }
    }
}

pub struct SimpleAttributePlugin;

pub struct Health;
pub struct Energy;
pub struct AttackSpeed;
pub struct MovementSpeed;

pub enum AttributeLabel {}

impl Plugin for SimpleAttributePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            basic_modifiers::<Max<Health>>
                .label("max_health_result")
                .before("health_result"),
        )
        .add_system(
            basic_modifiers::<Max<Health>>
                .label("max_health_result")
                .before("health_result"),
        )
        .add_system(basic_modifiers::<Health>.label("health_result"))
        .add_system(
            clamp_max::<Health>
                .label("health_max")
                .after("health_result"),
        );
    }
}

mod test {
    use super::*;
    use bevy::core::FixedTimestep;
    use std::time::Duration;

    pub struct Health;
    pub struct MovementSpeed;

    #[test]
    fn attribute_test() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_system_to_stage(
                CoreStage::Update,
                basic_modifiers::<Max<Health>>
                    .label("clamp_max_health")
                    .before("health"),
            )
            .add_system_to_stage(
                CoreStage::Update,
                basic_modifiers::<Health>.label("attribute_health"),
            )
            .add_system_to_stage(
                CoreStage::Update,
                clamp_max::<Health>
                    .label("health_max")
                    .after("attribute_health"),
            )
            .add_system_to_stage(
                CoreStage::Update,
                clamp_max::<MovementSpeed>
                    .label("movement_speed_max")
                    .after("attribute_movement_speed"),
            )
            .add_system_to_stage(
                CoreStage::Update,
                basic_modifiers::<MovementSpeed>.label("attribute_movement_speed"),
            );

        let health = app
            .world
            .spawn()
            .insert(Attribute::<Health>::new(Amount::from_num(11)))
            .insert(Attribute::<Base<Health>>::new(Amount::from_num(10)))
            .insert(Attribute::<Max<Health>>::new(Amount::from_num(10)))
            .id();

        let speed = app
            .world
            .spawn()
            .insert(Attribute::<MovementSpeed>::new(Amount::from_num(0)))
            .insert(Attribute::<Base<MovementSpeed>>::new(Amount::from_num(1)))
            .insert(Attribute::<Mult<MovementSpeed>>::new(Amount::from_num(1)))
            .insert(Attribute::<Add<MovementSpeed>>::new(Amount::from_num(1)))
            .id();

        assert_eq!(
            *app.world.get::<Attribute<Health>>(health).unwrap().amount(),
            Amount::from_num(11)
        );
        assert_eq!(
            *app.world
                .get::<Attribute<MovementSpeed>>(speed)
                .unwrap()
                .amount(),
            Amount::from_num(0)
        );

        app.update();
        assert_eq!(
            *app.world.get::<Attribute<Health>>(health).unwrap().amount(),
            Amount::from_num(10)
        );
        assert_eq!(
            *app.world
                .get::<Attribute<MovementSpeed>>(speed)
                .unwrap()
                .amount(),
            Amount::from_num(2)
        );
    }
}
