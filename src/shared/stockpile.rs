use crate::utils::*;

// Cube

#[derive(Debug)]
pub struct Cube<K, V> {
    id: K,
    name: String,
    pub data: Vec<V>,
}

impl<K, V> Cube<K, V> 
where K: Clone
{
    pub fn new(id: K) -> Self {
        Self {
            name: String::new(),
            id,
            data: Vec::new()
        }
    }
    pub fn with_name(id: K, name: String) -> Self {
        Self {
            name,
            id,
            data: Vec::new()
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn id(&self) -> K {
        self.id.clone()
    }
    pub fn insert(&mut self, index: usize, element: V) {
        self.data.insert(index, element)
    }
}

pub struct IntoIter<K, V>(Cube<K, V>);

impl<'a, K, V> Iterator for IntoIter<K, V> {
    type Item = V;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        self.0.data.pop()
    }
}


// CubeBox

pub struct CubeBox<K, V> {
    pub cube: Rc<RefCell<Cube<K, V>>>
}

impl<K, V> CubeBox<K, V> 
where K: Clone
{
    pub fn from(cube: Rc<RefCell<Cube<K, V>>>) -> Self {
        Self {
            cube
        }
    }
    pub fn id(&self) -> K {
        self.cube.borrow().id().clone()
    }
    pub fn insert(&mut self, index: usize, element: V) {
        self.cube.borrow_mut().data.insert(index, element)
    }
}

// impl<K, V> Deref for CubeBox<K, V> {
//     type Target = Cube<K, V>;
//     fn deref(&self) -> &<Self as std::ops::Deref>::Target { 
//         &self.cube
//     }
// }

// impl<K, V> DerefMut for CubeBox<K, V> {
//     fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target { todo!() }
// }

// Stockpile

pub struct Stockpile {
    pub map: HashMap<u8, Rc<RefCell<Cube<u8, String>>>>,
    pub cube_box: CubeBox<u8, String>
}

impl Stockpile {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        let id = 0;
        map.insert(id, Rc::new(RefCell::new(Cube::new(id))));
        let cube_arc = map.get(&id).expect("Just added with id.").clone();
        Self {
            map,
            cube_box: CubeBox::from(cube_arc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_stockpile() {
        let mut stockpile = Stockpile::new();
        {
            let mut cube = stockpile.map.get_mut(&0).unwrap().borrow_mut();
            cube.insert(0, format!("Hi."));
        }
        let mut cube_box = stockpile.cube_box;
        cube_box.insert(1, format!("Bye."));
        println!("{:#?}", cube_box.cube.borrow())
    }
}
