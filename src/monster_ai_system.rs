use super::{Map, Monster, Name, Position, Viewshed};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            // monsterがplayerを視界に捉えたら追いかける
            if viewshed.visible_tiles.contains(&*player_pos) {
                // console::logはrltkのhelper.
                // compile先が通常のプログラムかWASMか判別して出力する
                console::log(&format!("{} shouts insults!", name.name));

                // A*アルゴリズムで経路探索をする
                // monsterからplayerの経路
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );

                // 2step以上あればmonsterをpathの位置に移動させる
                // step[0](1step目)は現在地
                // 現状,playerからmonsterに重なることができてしまう
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
