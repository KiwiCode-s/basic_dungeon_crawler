use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Mana)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    //queries
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let mut mana_query = <&Mana>::query().filter(component::<Player>());

    let player_health = health_query.iter(ecs).nth(0).unwrap();
    let player_mana = mana_query.iter(ecs).nth(0).unwrap();

    //draw batch drawing to the third console layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    //Health Bar + Label   Order is important
    draw_batch.bar_horizontal(
        Point::new(0, 0),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, WHITE),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, BLACK),
    );

    //Mana Bar + Label
    draw_batch.bar_horizontal(
        Point::new(0, 1),
        SCREEN_WIDTH * 2,
        player_mana.current,
        player_mana.max,
        ColorPair::new(BLUE, WHITE),
    );
    draw_batch.print_color_centered(
        1,
        format!("Mana: {} / {}", player_mana.current, player_mana.max),
        ColorPair::new(WHITE, BLACK),
    );

    //Player Directions
    draw_batch.print_centered(3, "Explore the Dungeon. Cursor key to move.");
    //submit batch
    draw_batch.submit(1000).expect("Batch error");
}
