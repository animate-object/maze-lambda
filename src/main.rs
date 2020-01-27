use lambda_http::{lambda, Body, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_derive::Deserialize;
use serde_json;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct MazeRequest {
    dimensions: Dimensions,

    #[serde(rename(deserialize = "algorithm"))]
    #[serde(rename(deserialize = "alg"))]
    algorithm: Algorithm,

    #[serde(default)]
    corner: Option<Corner>,

    #[serde(default)]
    direction: Option<Direction>,
}

#[derive(Deserialize, Debug)]
struct Dimensions {
    height: usize,
    width: usize,
}

#[derive(Deserialize, Debug)]
enum Algorithm {
    #[serde(rename(deserialize = "ab"))]
    #[serde(rename(deserialize = "aldous-broder"))]
    AldousBroder,
    #[serde(rename(deserialize = "bt"))]
    #[serde(rename(deserialize = "binary-tree"))]
    BinaryTree,
    #[serde(rename(deserialize = "sw"))]
    #[serde(rename(deserialize = "side-winder"))]
    SideWinder,
}

#[derive(Deserialize, Debug)]
enum OutputType {
    #[serde(rename(deserialize = "bin"))]
    #[serde(rename(deserialize = "binary"))]
    BIN,
    #[serde(rename(deserialize = "ascii"))]
    ASCII,
}

#[derive(Deserialize, Debug)]
enum Direction {
    #[serde(rename(deserialize = "n"))]
    #[serde(rename(deserialize = "north"))]
    NORTH,
    #[serde(rename(deserialize = "e"))]
    #[serde(rename(deserialize = "east"))]
    EAST,
    #[serde(rename(deserialize = "s"))]
    #[serde(rename(deserialize = "south"))]
    SOUTH,
    #[serde(rename(deserialize = "w"))]
    #[serde(rename(deserialize = "west"))]
    WEST,
}

impl Direction {
    fn convert(&self) -> maze::Direction {
        match &self {
            Direction::NORTH => maze::Direction::North,
            Direction::EAST => maze::Direction::East,
            Direction::SOUTH => maze::Direction::South,
            Direction::WEST => maze::Direction::West,
        }
    }
}

#[derive(Deserialize, Debug)]
enum Corner {
    #[serde(rename(deserialize = "nw"))]
    #[serde(rename(deserialize = "northwest"))]
    NORTHWEST,
    #[serde(rename(deserialize = "ne"))]
    #[serde(rename(deserialize = "northeast"))]
    NORTHEAST,
    #[serde(rename(deserialize = "se"))]
    #[serde(rename(deserialize = "southeast"))]
    SOUTHEAST,
    #[serde(rename(deserialize = "sw"))]
    #[serde(rename(deserialize = "southwest"))]
    SOUTHWEST,
}

impl Corner {
    fn convert(&self) -> maze::Corner {
        match &self {
            Corner::NORTHWEST => maze::Corner::NorthWest,
            Corner::NORTHEAST => maze::Corner::NorthEast,
            Corner::SOUTHWEST => maze::Corner::SouthWest,
            Corner::SOUTHEAST => maze::Corner::SouthEast,
        }
    }
}

impl MazeRequest {
    fn to_maze_args(&self) -> maze::Args {
        maze::Args {
            dimensions: self.derive_dimensions(),
            algorigthm: self.derive_algorithm(),
            output_type: maze::OutputType::BIN,
        }
    }

    fn derive_dimensions(&self) -> maze::Dimensions {
        maze::Dimensions {
            height: self.dimensions.height,
            width: self.dimensions.width,
        }
    }

    fn derive_algorithm(&self) -> maze::Algorithm {
        match self.algorithm {
            Algorithm::AldousBroder => maze::Algorithm::AlduousBroder,
            Algorithm::BinaryTree => maze::Algorithm::BinTree {
                start_corner: self.derive_corner(),
            },
            Algorithm::SideWinder => maze::Algorithm::SideWinder {
                traversal_direction: self.derive_direction(),
            },
        }
    }

    fn derive_corner(&self) -> maze::Corner {
        self.corner.as_ref().unwrap_or(&Corner::NORTHWEST).convert()
    }

    fn derive_direction(&self) -> maze::Direction {
        self.direction
            .as_ref()
            .unwrap_or(&Direction::NORTH)
            .convert()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

fn handler(req: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let body_str: &String = match req.body() {
        Body::Text(text) => text,
        _ => return Err(HandlerError::from("Invalid request body")),
    };
    println!("{}", body_str);

    let maze_req: MazeRequest = serde_json::from_str(body_str)?;

    println!("{:?}", maze_req);

    match maze::generate(maze_req.to_maze_args()).expect("Failure generating maze") {
        maze::Output::BIN(bytes) => Ok(bytes),
        _ => Err(HandlerError::from("Unsupported output type")),
    }
}
