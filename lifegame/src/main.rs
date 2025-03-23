use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rand::{Rng, thread_rng};

struct GameOfLife {
    grille: Vec<Vec<bool>>,
    largeur: usize,
    hauteur: usize,
}

impl GameOfLife {
    fn new(largeur: usize, hauteur: usize) -> Self {
        let grille = vec![vec![false; largeur]; hauteur];
        GameOfLife { grille, largeur, hauteur }
    }

    fn initialiser(&mut self) {
        let mut rng = rng();
        for i in 0..self.hauteur {
            for j in 0..self.largeur {
                self.grille[i][j] = rng.random_bool(0.5);
            }
        }
    }

    // ... rest of the code remains the same ...
}
