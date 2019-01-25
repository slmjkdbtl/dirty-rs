// wengwengweng

use dirty::*;
use specs::*;

use crate::trans::*;
use crate::vel::*;

pub struct MoveSys;

impl<'a> System<'a> for MoveSys {

	type SystemData = (
		ReadStorage<'a, Vel>,
		WriteStorage<'a, Trans>
	);

	fn run(&mut self, (vel, mut trans): Self::SystemData) {
		for (vel, trans) in (&vel, &mut trans).join() {
			trans.pos = trans.pos + vel.pos;
		}
	}

}
