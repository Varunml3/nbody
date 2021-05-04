#![allow(non_snake_case, dead_code)]
#![feature(allocator_api, allocator_internals)]

use std::{cell::UnsafeCell, cmp::max, convert::TryInto, sync::atomic::{AtomicBool, Ordering::*}};
use float_ord::FloatOrd;
use particle::Particle;
use atomic_float::AtomicF32;
use snmalloc_rs::SnMalloc;
use parking_lot::{RawRwLock, RwLock, lock_api::RawRwLock as RawRwLockTrait};
use wgpu::{BackendBit, Instance};

mod particle;
mod render;
mod octree;

#[macro_use]
extern crate const_env;

#[macro_use]
extern crate nbody_macros;

extern crate tokio;

#[from_env]
pub const PARTICLES: usize = 100;

#[from_env]
pub const THREADS: u8 = 1;

pub const HORIZONTAL_DENSITY_RESOLUTION: usize = 16;
pub const VERTICAL_DENSITY_RESOLUTION: usize = 16;

pub const DENSITY_MATRIX_LEN: usize = HORIZONTAL_DENSITY_RESOLUTION*VERTICAL_DENSITY_RESOLUTION;

pub const PER_THREAD_ARRAY_LEN: usize = PARTICLES / THREADS as usize;

#[global_allocator]
static ALLOCATOR: SnMalloc = SnMalloc;

pub static PARTICLE_ARRAY: RwLock<[Particle; PARTICLES]> = rwlock([Particle::const_default(); PARTICLES]);
pub static DENSITY_MATRIX: RwLock<[f32; DENSITY_MATRIX_LEN]> = rwlock([0.0; DENSITY_MATRIX_LEN]);

pub static MAX_X: AtomicF32 = AtomicF32::new(0.0);
pub static MAX_Y: AtomicF32 = AtomicF32::new(0.0);
pub static MAX_Z: AtomicF32 = AtomicF32::new(0.0);

pub static RUNNING: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() {
    assert_eq!(PARTICLES % (THREADS*8) as usize, 0);

    let instance = Instance::new(BackendBit::PRIMARY);
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let mut flags = wgpu::ShaderFlags::VALIDATION;
    match adapter.get_info().backend {
        wgpu::Backend::Vulkan | wgpu::Backend::Metal | wgpu::Backend::Gl => {
            flags |= wgpu::ShaderFlags::EXPERIMENTAL_TRANSLATION;
        }
        _ => {}
    }

    let cs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        flags,
    });
    

    let handles = start_threads!{
        println!("{}", THREAD_NUM);

        const START: usize = PER_THREAD_ARRAY_LEN * THREAD_NUM;
        const END: usize = START + PER_THREAD_ARRAY_LEN;

        loop {
            let read_lock = PARTICLE_ARRAY.read();

            let particles: [Particle; PER_THREAD_ARRAY_LEN] = {
                let ar: &[Particle] = &read_lock[START..END];
                match ar.try_into() {
                    Ok(s) => s,
                    Err(e) => panic!("Miscalculated compile time constant PER_THREAD_ARRAY_LEN"),
                }
            };

            let mut local_x_max = FloatOrd(0.0);
            let mut local_y_max = FloatOrd(0.0);
            let mut local_z_max = FloatOrd(0.0);

            for particle in particles.iter() {
                local_x_max = max(local_x_max, FloatOrd(particle.x));
                local_y_max = max(local_y_max, FloatOrd(particle.y));
                local_z_max = max(local_z_max, FloatOrd(particle.z));
            }
        
            std::mem::drop(read_lock);

            if !RUNNING.load(Relaxed) {
                break;
            }
        }
    };

    for h in handles {
        h.join().expect("Thread Panicked");
    }
}

pub const fn rwlock<T>(val: T) -> RwLock<T> {
    RwLock::const_new(RawRwLock::INIT, val)
}

// pub fn array_from_slice<T, const N:usize>(slice: &[T]) -> [T; N] {
//     slice.try_into().unwrap()
// }