use macroquad::prelude::*;
use crate::animations as anim;

pub struct Hero {
    bullets: Vec<(f32, f32, anim::Animator)>,
    bullets_timer: f32,
    hero: anim::Animator,
}

impl Hero {
    pub async fn new() -> Self {
        let mut animator = anim::Animator::new(); // Create animator
        animator.load("SpaceshipKit.png").await; // Load spritesheet

        let frames = vec![
            (Rect::new(0., 0., 36., 52.), 0.1),
            (Rect::new(37., 0., 36., 52.), 0.1),
            (Rect::new(74., 0., 36., 52.), 0.1),
        ];

        animator.add_frames(frames);
        Self {
            bullets: vec![],
            bullets_timer: 0.,
            hero: animator,
        }
    }

    pub fn draw(&self) {
        // Draw bullets
        for i in 0..self.bullets.len() {
            self.bullets[i].2.draw(self.bullets[i].0, self.bullets[i]. 1);
        }

        let sprite_rect: Rect = self.hero.rects[self.hero.current_frame].0;   
        self.hero.draw(
            mouse_position().0 - sprite_rect.w / 2., 
            mouse_position().1 - sprite_rect.h / 2. - 40.,
        );
    }

    async fn input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) && self.bullets_timer >= 0.3 {
            self.bullets_timer = 0.;

            let mut animator = anim::Animator::new(); // Create animator
            animator.load("Bullets.png").await; // Load spritesheet
    
            let frames = vec![
                (Rect::new(0., 0., 36., 52.), 0.1),
                (Rect::new(37., 0., 36., 52.), 0.1),
                (Rect::new(74., 0., 36., 52.), 0.1),
            ];
    
            animator.add_frames(frames);

            let sprite_rect: Rect = self.hero.rects[self.hero.current_frame].0;   
            self.bullets.push((
                mouse_position().0 - sprite_rect.w / 2., 
                mouse_position().1 - sprite_rect.h / 2. - 70.,
                animator,
            ));
        }
    }
    
    fn collision(&mut self) {
        // Do it in reverse order to avoid skipping elements
        for i in (0..self.bullets.len()).rev() {
            if self.bullets[i].1 <= 0. { self.bullets.remove(i); }
        }
    }

    pub async fn update(&mut self) {
        self.collision();
        self.input().await;

        self.bullets_timer = self.bullets_timer + get_frame_time();

        for i in 0..self.bullets.len() {
            self.bullets[i].1 = self.bullets[i].1 - (400. * get_frame_time());
            self.bullets[i].2.update();
        }

        
        self.hero.update();
    }
}