extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

mod tuple;

use tuple::Tuple;

struct Projectile {
  position: Tuple,
  velocity: Tuple,
}

struct Environment {
  gravity: Tuple,
  wind: Tuple,
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("projectiles", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();

  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump().unwrap();

  let env = Environment {
    gravity: Tuple::new_vector(0.0, -0.1, 0.0),
    wind: Tuple::new_vector(0.01, 0.0, 0.0),
  };
  let mut p = Projectile {
    position: Tuple::new_point(0.0, 1.0, 0.0),
    velocity: Tuple::new_vector(1.0, 1.0, 0.0),
  };
  let screen_offset = Tuple::new_vector(0.0, -20.0, 0.0);

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    tick(&env, &mut p);
    println!("y: {}", p.position.y);
    println!("y: {}", p.velocity.y);

    let draw_position = p.position.add(&screen_offset);
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_point(Point::new(
      draw_position.x as i32,
      -1 * draw_position.y as i32,
    ));

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.draw_line(Point::new(10, 10), Point::new(10, 600));
    canvas.present();
  }
}

fn tick(env: &Environment, projectile: &mut Projectile) {
  if projectile.position.y < -600.0 {
    projectile.velocity = Tuple::new_vector(projectile.velocity.x, -projectile.velocity.y, 0.0);
  }

  projectile.position = projectile.position.add(&projectile.velocity);
  projectile.velocity = projectile.velocity.add(&env.gravity).add(&env.wind);
}
