
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
        for i in 0..self.hauteur {
            for j in 0..self.largeur {
                self.grille[i][j] = rand::random::<bool>();
            }
        }
    }

    fn afficher(&self) {
        for i in 0..self.hauteur {
            for j in 0..self.largeur {
                if self.grille[i][j] {
                    print!("* ");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }

    fn compter_voisins(&self, i: usize, j: usize) -> usize {
        let mut voisins = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let ni = (i as i32 + x) as usize;
                let nj = (j as i32 + y) as usize;
                if ni < self.hauteur && nj < self.largeur {
                    if self.grille[ni][nj] {
                        voisins += 1;
                    }
                }
            }
        }
        voisins
    }

    fn etape(&mut self) {
        let mut nouvelle_grille = vec![vec![false; self.largeur]; self.hauteur];
        for i in 0..self.hauteur {
            for j in 0..self.largeur {
                let voisins = self.compter_voisins(i, j);
                if self.grille[i][j] {
                    nouvelle_grille[i][j] = voisins == 2 || voisins == 3;
                } else {
                    nouvelle_grille[i][j] = voisins == 3;
                }
            }
        }
        self.grille = nouvelle_grille;
    }
}

fn main() {
    let mut jeu = GameOfLife::new(20, 20);
    jeu.initialiser();
    loop {
        jeu.afficher();
        jeu.etape();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
