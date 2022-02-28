use cgmath::{Vector2};
use storm::*;
use storm::{graphics::shaders::text::{TextShaderPass, TextShader}, fontdue::Font, color::RGBA8, math::AABB2D};
use storm::graphics::shaders::text::{Text};
use storm::fontdue::layout::LayoutSettings;
use crate::*;
use super::*;


pub struct Button {
    pub started_click: bool,
    pub confirmed_click: bool,
    pub bounds: AABB2D
}

impl Button {
    pub fn was_clicked_on(&mut self, position: Vector2<f32>) {
        if self.bounds.contains_point(&position) {
            self.started_click = true;
        }
    }

    pub fn was_released_on(&mut self, position: Vector2<f32>) {
        if self.started_click == false {
            return;
        }

        if self.bounds.contains_point(&position) {
            self.confirmed_click = true;
        }
        else {
            self.confirmed_click = false;
        }
    }
}

pub struct Menu {
    text_shader_pass: TextShaderPass,
    text_shader: TextShader,
    fonts: [Font; 1],
    button: Button,
    button_sprites: [Sprite;1],
    button_shader_pass: SpriteShaderPass,
    sprite_shader: SpriteShader

}

impl Menu {
    pub fn new(ctx: &mut Context<FighthingApp>) -> Menu {
        let (text_shader_pass, text_shader) = setup_round_timer_text(ctx);
        let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
        let (button_sprites, button_shader_pass) = setup_join_game_button(ctx);
        let button_x = -250.0;
        let button_y = -300.0;
        let button_x_max = button_x + 500.0;
        let button_y_max = button_y + 600.0;

        Menu {
            text_shader_pass,
            text_shader,
            fonts,
            button: Button {
                started_click: false,
                confirmed_click: false,
                bounds: AABB2D::new(button_x, button_y, button_x_max, button_y_max)
            },
            button_sprites,
            button_shader_pass,
            sprite_shader: SpriteShader::new(ctx)
        }
    }

    pub fn files_needed_to_start() -> Vec<String> {
        return vec![
            String::from("../resources/");
        ];
    }

    pub fn tick(&mut self, ctx: &mut Context<FighthingApp>) -> GameState {
        if self.button.confirmed_click  {
            return GameState::Game;
        }
        ctx.clear(ClearMode::color_depth(RGBA8::BLACK));


        self.button_sprites[0].pos.x = self.button.bounds.min.x;
        self.button_sprites[0].pos.y = self.button.bounds.min.y;
        self.button_sprites[0].size.x = (self.button.bounds.max.x - self.button.bounds.min.x) as u16;
        self.button_sprites[0].size.y = (self.button.bounds.max.y - self.button.bounds.min.y) as u16;
        if self.button.started_click {
            self.button_sprites[0].color = RGBA8::RED;
        }
        else {
            self.button_sprites[0].color = RGBA8::WHITE;
        }

        self.button_shader_pass.buffer.set(&self.button_sprites);
        self.button_shader_pass.draw(&self.sprite_shader);

        self.text_shader_pass.clear_text();
        let layout_settings = LayoutSettings {
            x: 0.0,
            y: 420.0,
            max_width: Some(2000.0),
            ..Default::default()
        };
        self.text_shader_pass.append(
            &self.fonts,
            &layout_settings,
            &[Text {
                text: "Find a match?",
                font_index: 0,
                px: 200.0,
                color: RGBA8::WHITE,
                depth: 0.0,
        }]);
        self.text_shader_pass.draw(&self.text_shader);
        return GameState::Menu;
    }

    pub fn mouse_down(&mut self, position: Vector2<f32>) {
        self.button.was_clicked_on(position);
    }

    pub fn mouse_up(&mut self, position: Vector2<f32>) {
        self.button.was_released_on(position);
    }
}