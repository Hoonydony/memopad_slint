use slint::{Model, VecModel, ModelRc};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = TabDemo::new()?;
    
    ui.set_current_content(ui.get_tab_contents().row_data(0).unwrap_or("".into()));

    // 탭 내용 실시간 업데이트
    let ui_handle = ui.as_weak();
    ui.on_update_tab_content({
        let ui = ui_handle.clone();
        move |content: slint::SharedString| {
            if let Some(ui) = ui.upgrade() {
                let contents = ui.get_tab_contents();
                let contents_model = contents.as_any()
                    .downcast_ref::<VecModel<slint::SharedString>>()
                    .unwrap();
                contents_model.set_row_data(ui.get_current_tab() as usize, content.clone());
            }
        }
    });

    // Add tab
    let ui_handle = ui.as_weak();
    ui.on_add_new_tab({
        let ui = ui_handle.clone();
        move || {
            let ui = ui.unwrap();
            let new_tab_count = ui.get_tab_titles().row_count() + 1;
            let mut titles = ui.get_tab_titles().iter().collect::<Vec<_>>();
            let mut contents = ui.get_tab_contents().iter().collect::<Vec<_>>();
            titles.push(format!("Tab {}", new_tab_count).into());
            contents.push("".into());
            ui.set_tab_titles(ModelRc::new(VecModel::from(titles)));
            ui.set_tab_contents(ModelRc::new(VecModel::from(contents)));
            ui.set_current_tab((new_tab_count - 1) as i32);
            ui.set_current_content("".into());
            ui.set_is_editing(true);
            ui.set_is_bold(false);
            println!("Added new tab: Tab {}", new_tab_count);
        }
    });

    // Close tab
    let ui_handle = ui.as_weak();
    ui.on_close_tab({
        let ui = ui_handle.clone();
        move || {
            let ui = ui.unwrap();
            let current = ui.get_current_tab();
            if ui.get_tab_titles().row_count() > 1 && current >= 0 {
                let mut titles = ui.get_tab_titles().iter().collect::<Vec<_>>();
                let mut contents = ui.get_tab_contents().iter().collect::<Vec<_>>();
                titles.remove(current as usize);
                contents.remove(current as usize);
                let new_index = if current >= titles.len() as i32 { current - 1 } else { current };
                let new_content = if new_index >= 0 { contents[new_index as usize].clone() } else { "".into() };
                ui.set_tab_titles(ModelRc::new(VecModel::from(titles)));
                ui.set_tab_contents(ModelRc::new(VecModel::from(contents)));
                ui.set_current_tab(new_index);
                ui.set_current_content(new_content);
                ui.set_is_editing(true);
                ui.set_is_bold(false);
                println!("Closed tab at index: {}", current);
            }
        }
    });

    // Tab changed
    let ui_handle = ui.as_weak();
    ui.on_current_tab_changed({
        let ui = ui_handle.clone();
        move |new_index: i32| {
            if let Some(ui) = ui.upgrade() {
                if new_index >= 0 && new_index < ui.get_tab_contents().row_count() as i32 {
                    let current_content = ui.get_current_content();
                    let current_tab = ui.get_current_tab();
                    if current_tab >= 0 {
                        let contents = ui.get_tab_contents();
                        let contents_model = contents.as_any()
                            .downcast_ref::<VecModel<slint::SharedString>>()
                            .unwrap();
                        contents_model.set_row_data(current_tab as usize, current_content.clone());
                    }
                    let new_content = ui.get_tab_contents().row_data(new_index as usize).unwrap_or("".into());
                    ui.set_current_content(new_content);
                    ui.set_is_editing(true);
                }
            }
        }
    });

    // Font size
    let ui_handle = ui.as_weak();
    ui.on_apply_font_size({
        let ui = ui_handle.clone();
        move |size: f32| {
            if let Some(ui) = ui.upgrade() {
                ui.set_font_size(size.floor() as i32);
            }
        }
    });

    // Bold
    let ui_handle = ui.as_weak();
    ui.on_apply_bold({
        let ui = ui_handle.clone();
        move || {
            if let Some(ui) = ui.upgrade() {
                ui.set_is_bold(!ui.get_is_bold());
            }
        }
    });

    // Underline
    let ui_handle = ui.as_weak();
    ui.on_apply_underline({
        let ui = ui_handle.clone();
        move || {
            if let Some(ui) = ui.upgrade() {
                let text = ui.get_current_content();
                let styled = format!("<u>{}</u>", text);
                ui.set_current_content(styled.clone().into());
                let contents = ui.get_tab_contents();
                let contents_model = contents.as_any()
                    .downcast_ref::<VecModel<slint::SharedString>>()
                    .unwrap();
                contents_model.set_row_data(ui.get_current_tab() as usize, styled.into());
                println!("Underline applied as <u> tag (not visually supported in Slint 1.10)");
            }
        }
    });

    // Rename tab
    let ui_handle = ui.as_weak();
    ui.on_rename_tab({
        let ui = ui_handle.clone();
        move |index: i32, new_name: slint::SharedString| {
            if let Some(ui) = ui.upgrade() {
                if index >= 0 && index < ui.get_tab_titles().row_count() as i32 {
                    let titles = ui.get_tab_titles();
                    let titles_model = titles.as_any()
                        .downcast_ref::<VecModel<slint::SharedString>>()
                        .unwrap();
                    titles_model.set_row_data(index as usize, new_name.clone());
                    println!("Renamed tab at index {} to: {}", index, new_name);
                    // 편집 모드 종료는 Slint에서 처리되므로 추가 작업 불필요
                }
            }
        }
    });

    ui.run()
}