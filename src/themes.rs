use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub add_tab_bg: Color,
    pub arrow_btn_bg: Color,
    pub arrow_connector_size: f32,
    pub arrow_connector: Color,
    pub arrow: Color,
    pub bottom_panel_bg: Color,
    pub btn_border: Color,
    pub canvas_bg_color: Option<Color>,
    pub canvas_bg_line_color: Color,
    pub celebrate_btn_bg: Color,
    pub celebrate_btn: Color,
    pub drawing_pencil_btn_bg: Color,
    pub drawing_pencil_btn: Color,
    pub drawing_selected: Color,
    pub clipboard_image_bg: Color,
    pub code_default_lang: String,
    pub code_theme: String,
    pub color_change_1: Color,
    pub color_change_2: Color,
    pub color_change_3: Color,
    pub color_change_4: Color,
    pub color_change_5: Color,
    pub del_button: Color,
    pub doc_list_bg: Color,
    pub font_name: String,
    pub font_size: f32,
    pub font: Color,
    pub front_back_btn_bg: Color,
    pub inline_code: Color,
    pub left_panel_bg: Color,
    pub line_height: f32,
    pub link: Color,
    pub max_camera_space: f32,
    pub menu_bg: Color,
    pub menu_btn_bg: Color,
    pub menu_btn: Color,
    pub modal_bg: Color,
    pub modal_text_input_bg: Color,
    pub new_tab_btn_bg: Color,
    pub node_bg: Color,
    pub node_border: Color,
    pub node_found_color: Color,
    pub node_height: f32,
    pub node_manipulation_bg: Color,
    pub node_manipulation: Color,
    pub node_shadow: Color,
    pub node_width: f32,
    pub ok_cancel_bg: Color,
    pub paper_node_bg: Color,
    pub resize_marker_size: f32,
    pub search_box_bg: Color,
    pub search_box_border: Color,
    pub selected_node_border: Color,
    pub shadow: Color,
    pub tab_bg: Color,
    pub text_pos_btn_bg: Color,
    pub tooltip_bg: Color,
}

pub fn velo_light() -> Theme {
    Theme {
        add_tab_bg: Color::rgb(1., 193.0 / 255.0, 7.0 / 255.0),
        arrow_btn_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        arrow_connector_size: 5.0,
        arrow_connector: Color::BLUE.with_a(0.2),
        arrow: Color::rgb(63.0 / 255.0, 81.0 / 255.0, 181.0 / 255.0),
        bottom_panel_bg: Color::rgb(189.0 / 255.0, 189.0 / 255.0, 189.0 / 255.0),
        btn_border: Color::rgb(0.5, 0.5, 0.5),
        canvas_bg_color: None,
        canvas_bg_line_color: Color::rgba(97. / 255., 164. / 255., 1., 0.2),
        celebrate_btn_bg: Color::rgb(0.9, 0.9, 0.9),
        celebrate_btn: Color::RED,
        drawing_pencil_btn_bg: Color::rgb(0.9, 0.9, 0.9),
        drawing_pencil_btn: Color::RED,
        clipboard_image_bg: Color::WHITE,
        code_default_lang: "rs".to_string(),
        code_theme: "Solarized (light)".to_string(),
        color_change_1: Color::NONE,
        color_change_2: Color::rgb(215.0 / 255.0, 204.0 / 255.0, 200.0 / 255.0),
        color_change_3: Color::rgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0),
        color_change_4: Color::rgb(239., 68.0 / 255.0, 68.0 / 255.0),
        color_change_5: Color::rgb(34.0 / 255.0, 197.0 / 255.0, 94.0 / 255.0),
        del_button: Color::BLACK,
        doc_list_bg: Color::rgb(158., 158., 158.),
        font_name: "Victor Mono".to_string(),
        font_size: 14.,
        font: Color::rgb(0.0, 0.0, 0.0),
        front_back_btn_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        inline_code: Color::GRAY,
        left_panel_bg: Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
        line_height: 18.,
        link: Color::BLUE,
        menu_bg: Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0),
        menu_btn_bg: Color::rgb(0.0 / 255.0, 150.0 / 255.0, 136.0 / 255.0),
        menu_btn: Color::BLACK,
        modal_bg: Color::WHITE,
        modal_text_input_bg: Color::WHITE,
        new_tab_btn_bg: Color::rgb(189.0 / 255.0, 189.0 / 255.0, 189.0 / 255.0),
        node_bg: Color::rgb(0.98, 0.98, 0.98),
        node_border: Color::BLACK.with_a(0.8),
        node_found_color: Color::TEAL,
        node_height: 144.,
        node_manipulation_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        node_manipulation: Color::BLACK,
        node_shadow: Color::BLACK.with_a(0.5),
        node_width: 144.,
        ok_cancel_bg: Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0),
        paper_node_bg: Color::rgb(1., 236. / 255., 172. / 255.),
        resize_marker_size: 10.,
        search_box_bg: Color::WHITE,
        search_box_border: Color::GRAY.with_a(0.5),
        selected_node_border: Color::rgba(33.0 / 255.0, 150.0 / 255.0, 243.0 / 255.0, 1.0),
        shadow: Color::BLACK.with_a(0.5),
        tab_bg: Color::rgb(0.9, 0.9, 0.9),
        text_pos_btn_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        tooltip_bg: Color::rgb(1., 1., 1.),
        max_camera_space: 1_000_000_000_000_000_000.,
        drawing_selected: Color::BLUE,
    }
}

pub fn velo_dark() -> Theme {
    Theme {
        add_tab_bg: Color::rgb(0.2, 0.2, 0.2),
        arrow_btn_bg: Color::rgb(0.9, 0.9, 0.9),
        arrow_connector_size: 5.0,
        arrow_connector: Color::rgb(0.1, 0.1, 0.1),
        arrow: Color::rgb(0.8, 0.8, 0.8),
        bottom_panel_bg: Color::rgb(0.1, 0.1, 0.1),
        btn_border: Color::rgb(0.8, 0.8, 0.8),
        canvas_bg_color: Some(Color::rgb(0.3, 0.3, 0.3)),
        canvas_bg_line_color: Color::rgba(1., 0. / 255., 1., 0.2),
        celebrate_btn_bg: Color::GRAY,
        celebrate_btn: Color::RED,
        drawing_pencil_btn_bg: Color::GRAY,
        drawing_pencil_btn: Color::RED,
        clipboard_image_bg: Color::BLACK,
        code_default_lang: "rs".to_string(),
        code_theme: "base16-ocean.dark".to_string(),
        color_change_1: Color::NONE,
        color_change_2: Color::rgb(215.0 / 255.0, 204.0 / 255.0, 200.0 / 255.0),
        color_change_3: Color::rgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0),
        color_change_4: Color::rgb(239., 68.0 / 255.0, 68.0 / 255.0),
        color_change_5: Color::rgb(34.0 / 255.0, 197.0 / 255.0, 94.0 / 255.0),
        del_button: Color::WHITE,
        doc_list_bg: Color::rgb(0.2, 0.2, 0.2),
        font_name: "Source Code Pro".to_string(),
        font_size: 14.,
        font: Color::rgb(240. / 255.0, 240. / 255.0, 240. / 255.0),
        front_back_btn_bg: Color::rgb(0.9, 0.9, 0.9),
        inline_code: Color::WHITE,
        left_panel_bg: Color::GRAY,
        line_height: 18.,
        link: Color::BLUE,
        menu_bg: Color::GRAY,
        menu_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        menu_btn: Color::rgb(240. / 255.0, 240. / 255.0, 240. / 255.0),
        modal_bg: Color::rgb(0.2, 0.2, 0.2),
        modal_text_input_bg: Color::rgb(0.2, 0.2, 0.2),
        new_tab_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        node_bg: Color::DARK_GRAY,
        node_border: Color::rgb(0.3, 0.3, 0.3),
        node_found_color: Color::TEAL,
        node_height: 144.,
        node_manipulation_bg: Color::rgb(0.2, 0.2, 0.2),
        node_manipulation: Color::rgb(240. / 255.0, 240. / 255.0, 240. / 255.0),
        node_shadow: Color::BLUE.with_a(0.5),
        node_width: 144.,
        ok_cancel_bg: Color::rgb(0.2, 0.2, 0.2),
        paper_node_bg: Color::rgb(33. / 255., 33. / 255., 70. / 255.),
        resize_marker_size: 10.,
        search_box_bg: Color::rgb(0.25, 0.25, 0.25),
        search_box_border: Color::rgb(0.2, 0.2, 0.2),
        selected_node_border: Color::BLUE,
        shadow: Color::BLACK.with_a(0.5),
        tab_bg: Color::rgb(0.2, 0.2, 0.2),
        text_pos_btn_bg: Color::rgb(0.9, 0.9, 0.9),
        tooltip_bg: Color::rgb(0.2, 0.2, 0.2),
        max_camera_space: 1_000_000_000_000_000_000.,
        drawing_selected: Color::BLUE,
    }
}

pub fn get_theme_by_name(theme_name: &str) -> Theme {
    match theme_name {
        "light" => velo_light(),
        "dark" => velo_dark(),
        _ => velo_light(),
    }
}
