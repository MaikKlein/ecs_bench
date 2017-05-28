#![feature(test)]

extern crate test;
use test::Bencher;

extern crate froggy;
extern crate ecs_bench;

use froggy::{Pointer, Storage};

use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};

struct Entity {
    pos: Pointer<Position>,
    vel: Option<Pointer<Velocity>>,
}

struct World {
    pos: Storage<Position>,
    vel: Storage<Velocity>,
    entities: Vec<Entity>,
}

fn build() -> World {
    let mut world = World {
        pos: Storage::with_capacity(N_POS),
        vel: Storage::with_capacity(N_POS),
        entities: Vec::with_capacity(N_POS),
    };

    // setup entities
    for i in 0 .. N_POS {
        let vel = if i % N_POS_PER_VEL == 0 {
            Some(world.vel.create(Velocity { dx: 0.0, dy: 0.0 }))
        } else {
            None
        };
        world.entities.push(Entity {
            pos: world.pos.create(Position { x: 0.0, y: 0.0 }),
            vel: vel,
        });
    }

    world.pos.sync_pending();
    world.vel.sync_pending();
    world
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();

    b.iter(|| {
        // update
        for e in &world.entities {
            if let Some(ref vel) = e.vel {
                let mut p = &mut world.pos[&e.pos];
                let v = world.vel[vel];
                p.x += v.dx;
                p.y += v.dy;
            }
        }
        // render
        for _p in &world.pos {}
    });
}
