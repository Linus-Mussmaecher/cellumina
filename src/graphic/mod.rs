/// Contains the [displayer::AutomatonDisplayer] struct.
mod view;
pub(crate) use view::run_live;
/// Contains the [vertex::Vertex] struct and some related const objects.
mod vertex;

mod controller;
use controller::AutomatonController;

mod model;
use model::AutomatonModel;
