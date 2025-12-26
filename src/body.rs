mod dice;
use dice::Die;

pub struct Body {
    name: String,
    die: Die
}

impl Body {
    /// Creates a new `Body` instance with a Character name and default values.
    pub fn new(name: String, die: Die) -> Body {
        Body { name, die }
    }
}
