//struct (mesmo que Class em java)

struct Fila<T> {
    pessoas: Vec<T>,
}
//impl (implementar o paciente na fila)
impl<T> Fila<T> {
    fn nova() -> Self {
        Self {
            pessoas: Vec::new(),
        }
    }

    // Funçao adicionar pessoa na proprio fila (&mut para ser mutavel)(self referencia a propria fila)
    fn esperar(&mut self, item: T) {
        self.pessoas.push(item);
    }

    // Funçao remove o paciente atendido da fila
    fn atender(&mut self) -> Option<T> {
        //Condiçao para saber se esta vazio (is_empty())
        if self.pessoas.is_empty() {
            None
        } else {
            // Removemos o primeiro elemento e retornamos
            Some(self.pessoas.remove(0))
        }
    }
    // Obtém o paciente do início da fila sem removê-lo
    fn frente(&self) -> Option<&T> {
        self.pessoas.first()
    }

    // Verifica se a fila está vazia (funçao retorna true ou falso)
    fn esta_vazia(&self) -> bool {
        self.pessoas.is_empty()
    }
    
}

fn main() {
    let mut fila = Fila::nova();
    
    fila.esperar("Renan");
    fila.esperar("Roberto");
    fila.esperar("Raphael");
    fila.esperar("Yago");

    println!("Paciente no início da fila: {:?}", fila.frente());

    while let Some(item) = fila.atender() {
        println!("Atendido: {}", item);
    }
    
    // Verificando se a fila está vazia após as remoções
    println!("Fila Vazia {}", fila.esta_vazia()); // true
}
