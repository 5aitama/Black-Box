#[derive(Default, Debug)]
pub struct Node {
    extend: (f32, f32, f32),
    center: (f32, f32, f32),
    childs: Box<[Option<Node>; 8]>,
}

impl Node {
    pub fn new(center: (f32, f32, f32), size: f32) -> Self {
        let s = size / 2.0;
        Self {
            extend: (s, s, s),
            center,
            childs: Box::new([
                None, None, None, None,
                None, None, None, None,
            ]),
        }
    }

    pub fn add_point(&mut self, point: (f32, f32, f32)) {
        

        if !self.contain(point) { return; }

        if self.extend.0 * 2.0 <= 1.0 && self.extend.1 * 2.0 <= 1.0 && self.extend.2 * 2.0 <= 1.0 {
            return;
        }

        let mut childs = Box::new(self.subdivide());

        for child in childs.as_mut() {
            if let Some(child) = child {
                child.add_point(point);
            }
        }

        self.childs = childs;
    }

    pub fn contain(&self, point: (f32, f32, f32)) -> bool {
        point.0 >= self.center.0 - self.extend.0 &&
        point.1 >= self.center.1 - self.extend.1 &&
        point.2 >= self.center.2 - self.extend.2 &&

        point.0 <= self.center.0 + self.extend.0 &&
        point.1 <= self.center.1 + self.extend.1 &&
        point.2 <= self.center.2 + self.extend.2
    }

    /// Subdivide the current [Node] into 8 sub [Node].
    pub fn subdivide(&self) -> [Option<Node>; 8] {
        let half_extend = (
            self.extend.0 / 2.0,
            self.extend.1 / 2.0,
            self.extend.2 / 2.0,
        );

        let xmin_ymin_zmin = (
            self.center.0 - half_extend.0,
            self.center.1 - half_extend.1,
            self.center.2 - half_extend.2,
        );

        let xmin_ymax_zmin = (
            self.center.0 - half_extend.0,
            self.center.1 + half_extend.1,
            self.center.2 - half_extend.2,
        );

        let xmax_ymax_zmin = (
            self.center.0 + half_extend.0,
            self.center.1 + half_extend.1,
            self.center.2 - half_extend.2,
        );

        let xmax_ymin_zmin = (
            self.center.0 + half_extend.0,
            self.center.1 - half_extend.1,
            self.center.2 - half_extend.2,
        );

        let xmin_ymin_zmax = (
            self.center.0 - half_extend.0,
            self.center.1 - half_extend.1,
            self.center.2 + half_extend.2,
        );

        let xmin_ymax_zmax = (
            self.center.0 - half_extend.0,
            self.center.1 + half_extend.1,
            self.center.2 + half_extend.2,
        );

        let xmax_ymax_zmax = (
            self.center.0 + half_extend.0,
            self.center.1 + half_extend.1,
            self.center.2 + half_extend.2,
        );

        let xmax_ymin_zmax = (
            self.center.0 + half_extend.0,
            self.center.1 - half_extend.1,
            self.center.2 + half_extend.2,
        );

        [
            Some(Node::new(xmin_ymin_zmin, self.extend.0)),
            Some(Node::new(xmin_ymax_zmin, self.extend.0)),
            Some(Node::new(xmax_ymax_zmin, self.extend.0)),
            Some(Node::new(xmax_ymin_zmin, self.extend.0)),

            Some(Node::new(xmin_ymin_zmax, self.extend.0)),
            Some(Node::new(xmin_ymax_zmax, self.extend.0)),
            Some(Node::new(xmax_ymax_zmax, self.extend.0)),
            Some(Node::new(xmax_ymin_zmax, self.extend.0)),
        ]
    }
}