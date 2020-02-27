use crate::data::Map;
use std::cmp::Eq;
use std::cmp::PartialEq;

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Track {
    last_point: Point,
    track: Vec<Point>,
}

pub fn turn_count(map: Map) {
    let mut create_routes: bool = true;
    let mut _start = Point { x: 0, y: 1 };
    let _end = Point { x: map.x - 1, y: map.y - 2 };
    let mut await_routes: Vec<Track> = Vec::new();
    let mut valid_routes: Vec<Vec<Point>> = Vec::new();
    await_routes.push(Track { last_point: _start, track: Vec::new() });

    while create_routes {
        let mut next_step: bool = true;
        let mut _current_points: Vec<Point> = Vec::new();
        if await_routes.len() == 0 {
            create_routes = false;
        } else {
            let alt_rout: Track = await_routes.remove(0);
            _current_points = alt_rout.track;
            _start = alt_rout.last_point;
            while next_step {
                let data_routes = find_route(&map, &mut _start);
                if data_routes.len() == 0 {
                    next_step = false;
                } else {
                    let mut new_start: bool = false;
                    _current_points.push(_start);
                    let mut rout_not_found: bool = true;
                    for single_rout in data_routes {
                        if !new_start {
                            if !_current_points.contains(&single_rout) {
                                _start = single_rout;
                                rout_not_found = false;
                                new_start = true;
                                if single_rout.x == _end.x && single_rout.y == _end.y {
                                    valid_routes.push(_current_points.to_vec());
                                    next_step = false;
                                }
                            }
                        } else {
                            if !_current_points.contains(&single_rout) {
                                if single_rout.x == _end.x && single_rout.y == _end.y {
                                    _current_points.push(single_rout);
                                    valid_routes.push(_current_points.to_vec());
                                    next_step = false;
                                } else {
                                    await_routes.push(Track { last_point: single_rout, track: _current_points.to_vec() });
                                }
                            }
                        }
                    }
                    if rout_not_found {
                        next_step = false;
                    }
                }
            }
        }
    }

    if valid_routes.len() > 0 {
        print!("Najmniejsza liczba zakrętów to: {}", check_turn(valid_routes));
    } else {
        print!("Labirynt nie ma drogi wyjścia.");
    }

}

fn find_route(map: &Map, point: &mut Point) -> Vec<Point> {
    let mut related_points: Vec<Point> = Vec::new();

    if (point.y + 1) < map.y {
        if map.data[point.y + 1][point.x] == 1 {
            related_points.push(Point { x: point.x, y: point.y + 1 });
        }
    }

    if point.y > 0 {
        if map.data[point.y - 1][point.x] == 1 {
            related_points.push(Point { x: point.x, y: point.y - 1 });
        }
    }

    if point.x > 0 {
        if map.data[point.y][point.x - 1] == 1 {
            related_points.push(Point { x: point.x - 1, y: point.y });
        }
    }

    if (point.x + 1) < map.x {
        if map.data[point.y][point.x + 1] == 1 {
            related_points.push(Point { x: point.x + 1, y: point.y });
        }
    }

    return related_points;
}

fn check_turn(routes: Vec<Vec<Point>>) -> i32 {
    let mut turns_number: i32 = 0;

    for rout in routes {
        let mut turns_count: i32 = 0;
        for (index, point) in rout.iter().enumerate() {
            if index < rout.len() - 2 {
                if point.y == rout[index + 1].y && point.y != rout[index + 2].y {
                    turns_count = turns_count + 1;
                } else if point.x == rout[index + 1].x && point.x != rout[index + 2].x {
                    turns_count = turns_count + 1;
                }
            }
        }

        if turns_number == 0 {
            turns_number = turns_count;
        } else if turns_number > turns_count {
            turns_number = turns_count;
        }
    }

    return turns_number;
}