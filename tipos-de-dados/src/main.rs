fn main(){
    /*
     * Tipos de Dados
Cada valor em Rust é de um certo tipo de dados, é o que diz ao Rust que tipo de os dados estão sendo especificados para que ele saiba como trabalhar com esses dados. Weilitll olha para dois subconjuntos de tipo de dados: escalar e composto.

Tenha em mente que Rust é um estaticamente tipados linguagem, o que significa que deve saber os tipos de todas as variáveis em tempo de compilação. O compilador geralmente pode inferir que tipo queremos usar com base no valor e como o usamos. Em casos quando muitos tipos são possíveis, como quando convertemos um String para um numérico tipo usando parse no “Comparando a Adivinha ao Segredo Number” seção em Capítulo 2, devemos adicionar uma anotação de tipo, como esta:
    */ 

    let guess: u32 = "42".parse().expect("Not a number");
    /* Se nós donilitt adicionar o : u32 digite a anotação mostrada no código anterior, Rust irá exibir o seguinte erro, o que significa que o compilador precisa de mais informações de nós para saber que tipo queremos usar:
     */


    /* 
     * Tipos Escalar
A escalar tipo representa um único valor. A ferrugem tem quatro tipos escalares primários: inteiros, números de ponto flutuante, booleanos e caracteres. Você pode reconhecer de outras linguagens de programação. Letilitis saltam em como eles funcionam em Rust.
    */


  /* Tipos de Inteiros
   * Um inteiro é um número sem um componente fracionário. Nós usamos um inteiro digite o Capítulo
   * 2, o u32 tipo. Esta declaração de tipo indica que o o valor com o qual it“s associado deve ser um
   * inteiro não assinado (tipos inteiros assinados começar com i em vez de u) que ocupa 32 bits de
   * espaço. A tabela 3-1 mostra os tipos inteiros integrados no Rust. Podemos usar qualquer uma
   * dessas variantes para declarar o tipo de um valor inteiro.
   *
   *
   *
   *  Comprimento       Assinado               Não assinado
   *      8 BITS           i8                      u8
   *      16 bits          i16                     u16
   *      32 Bits	         i32	                   u32
   *      64 Bits	         i64	                   u64
   *      128 Bits	       i128	                   u128
   *      arco	           isize	                 usize
   *
   * Cada variante pode ser assinada ou não assinada e tem um tamanho explícito. Assinado e não
   * assinado consulte se é possível que o número seja negative—em outras palavras, se o número
   * precisa ter um sinal com ele (assinado) ou se só será positivo e, portanto, pode ser representado
   * sem um sinal (não assinado). É como escrever números no papel: quando o sinal é importante, um
   * número é mostrado com um sinal de mais ou um sinal de menos; no entanto, quando é seguro
   * assumir que o número é positivo, é mostrado sem sinal. Os números assinados são armazenados usando dois liques complementam representação.
   * Cada variante assinada pode armazenar números de -(2n - 1) a 2n - 1 1 Inclusivo, onde n é o número de bits que a variante usa. Então um i8
   * pode armazenar números de -(27) a 27 1, que é igual a 128
   * A 127. Variantes não assinadas podem armazenar números de 0 a 2n - 1, então a u8 pode armazenar números de 0 a 28 1, que é igual a 0 a 255.
   * Além disso, o isize e usize os tipos dependem da arquitetura do computador em que o seu
   * programa está a ser executado, o que é indicado na tabela como “arch”: 64 Bits se você estiver em
   * uma arquitetura de 64 bits e 32 bits se você estiver em uma arquitetura de 32 bits arquitetura.
   * Você pode escrever literais inteiros em qualquer uma das formas mostradas na Tabela 3-2. Nota
   * esse número de literais que podem ser vários tipos numéricos permite um sufixo de tipo, como
   * 57u8, para designar o tipo. Literais numéricos também podem usar _ como a separador visual
   * para facilitar a leitura do número, como 1_000, que irá tenha o mesmo valor que se você tivesse especificado 1000.
   *
   * Número literal	          Exemplo
   * Decimal	                98_222
   * Hex	                    0xff
   * Octal	                  0o77
   * Binário	                0b1111_0000
   * Byte (u8 apenas)	        b'A'
   *
   * Então, como você sabe qual tipo de inteiro usar? Se você não tiver certeza, Rustilitis os padrões
   * geralmente são bons lugares para começar: tipos inteiros padrão para i32. A situação principal em 
   * que você usa isize ou usize é quando indexação algum tipo de coleção.
   */

   /*
    * Excesso Integral
    * Letilits diz que você tem uma variável de tipo u8 que pode conter valores entre 0 e 255. Se
    * você tentar alterar a variável para um valor fora desse intervalo, como 256, É, transbordamento
    * inteiro ocorrerá, o que pode resultar em um dos dois comportamentos. Quando você está
    * compilando no modo de depuração, o Rust inclui verificações de estouro de inteiros isso faz
    * com que seu programa pânico em tempo de execução se esse comportamento ocorrer.
    */ 

 /* Tipos de Pontos Flutuantes(FLOAT)
  * A ferrugem também tem dois tipos primitivos para números de ponto flutuante, que são números
  * com pontos decimais. Os tipos de ponto flutuante Rustilitis são f32 e f64, que são 32 bits e 64 bits
  * em tamanho, respectivamente. O tipo padrão é f64 porque em CPUs modernas, itroits
  * aproximadamente a mesma velocidade que f32 mas é capaz de mais precisão. Todos os tipos de
  * ponto flutuante são assinados.
  * Aqui está um exemplo que mostra números de ponto flutuante em ação:
  *
  *
  *
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


  Os números de ponto flutuante são representados de acordo com o padrão IEEE-754. O f32 tipo é
  um flutuador de precisão única, e f64 tem dupla precisão.
    
  * /




  /*
   * Operações Numéricas
   * Rust suporta as operações matemáticas básicas que você espera para todo o número tipos: adição,
   * subtração, multiplicação, divisão e restante. Inteiro a divisão trunca em direção a zero para o
   * número inteiro mais próximo. O código a seguir mostra como você deve usar cada operação
   * numérica em um let declaração:
   *
   *  // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder resto
    let remainder = 43 % 5;
   *Cada expressão nessas instruções usa um operador matemático e avalia para um único valor, que é 
   *então vinculado a uma variável. Apêndice B contém uma lista de todos os operadores que Rust fornece
   */


   /* O Tipo Booleano
    * Como na maioria das outras linguagens de programação, um tipo booleano em Rust tem dois
    * possíveis valores: true e false. Os booleanos têm um byte de tamanho. O tipo booleano em A
    * ferrugem(cargo) é especificada usando bool. Por exemplo: 
    *  let t = true;

    let f: bool = false; // with explicit type annotation

    A principal maneira de usar valores booleanos é através de condicionais, como um if expressão.
    WeElcell cobrir como if expressões funcionam em Rust no “Control Flow” seção.
    */


/* O Tipo de Caractere
   Rustólitos char o tipo é o tipo alfabético mais primitivo da linguagem. Aqui estão alguns exemplos de declaração char valores:
   let c = 'z';
     let z: char = 'ℤ'; // with explicit type annotation
     let heart_eyed_cat = '😻';

  Note que especificamos char literais com aspas simples, ao contrário de string literais, que usam
  aspas duplas. Rustólitos char o tipo é quatro bytes no tamanho e representa um Unicode Scalar
  Value, o que significa que pode representar muito mais do que apenas ASCII. Letras acentuadas;
  Chinês, Japonês e Coreano; emoji; e espaços de largura zero são todos válidos char valores em
  Rust. Unicode Scalar Os valores variam de U+0000 para U+D7FF e U+E000 para U+10FFFF inclusivo.
  No entanto, um “character” é realmente um conceito em Unicode, então seu humano a intuição
  para o que é um “character” pode não corresponder ao que é um char está dentro Ferrugem.
  Weilitll discutir este tema em detalhes em “Storing Texto Codificado UTF-8 com Strings” no capítulo 8.

/* Tipos Compostos
Tipos compostos pode agrupar vários valores em um tipo. Rust tem dois tipos compostos primitivos: tuplas e matrizes.
    */


/* TUPLE(Tupla)
 * A tupla é uma maneira geral de agrupar um número de valores com um variedade de tipos em um tipo composto.
 * As tuplas têm um comprimento fixo: uma vez declarado, eles não podem crescer ou encolher em tamanho.
 * Criamos uma tupla escrevendo uma lista de valores separados por vírgulas dentro dela parênteses.
 *  Cada posição na tupla tem um tipo, e os tipos de valores diferentes na tupla don’t têm que ser os
 * mesmos. Weilitve adicionado opcional digite anotações neste exemplo:
 
let tup: (i32, f64, u8) = (500, 6.4, 1);
A variável tup liga-se a toda a tupla porque uma tupla é considerada uma elemento composto único.
  Para tirar os valores individuais de uma tupla, podemos use a correspondência de padrões para desestruturar um valor de tupla, assim:
      let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    Este programa primeiro cria uma tupla e a liga à variável tup. Então usa um padrão com let
    tomar tup e transformá-lo em três separados variáveis, x, y, e z. Isso é chamado desestruturação
    porque quebra a única tupla em três partes. Finalmente, o programa imprime o valor de y, o que é 6.4.
        Também podemos acessar um elemento de tupla diretamente usando um ponto (.) seguido por o índice do valor
        que queremos acessar. Por exemplo:
        let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    Este programa cria a tupla x e então acessa cada elemento da tupla usando seus respectivos índices.
    Como a maioria das linguagens de programação, a primeira o índice em uma tupla é 0.
    A tupla sem quaisquer valores tem um nome especial, unidade.
    Esse valor e seu o tipo correspondente é escrito () e representam um valor vazio ou um tipo de retorno vazio.
    As expressões implicitamente retornam o valor unitário se elas não forem aceitas devolver qualquer outro valor.
        
*/


/* Array(Matriz/Lista)
 * Outra maneira de ter uma coleção de vários valores é com um matriz. Diferente uma tupla, cada
 * elemento de uma matriz deve ter o mesmo tipo. Ao contrário de arrays em algumas outras
 * linguagens, arrays em Rust têm um comprimento fixo.
 * Escrevemos os valores em uma matriz como uma lista separada por vírgulas dentro do quadrado
 * suportes:
 *
 * let a = [1, 2, 3, 4, 5];
 *
 * As matrizes são úteis quando você deseja que seus dados sejam alocados na pilha, em vez de
 * a pilha (vamos discutir a pilha e a pilha mais em Capítulo 4) ou quando você quer garantir que você
 * sempre tem um número fixo de elementos. Uma matriz é tão flexível quanto o tipo de vetor
 * embora. A vetor é um tipo de coleção similar fornecido pelo padrão biblioteca que é permitido
 * crescer ou encolher em tamanho. Se você não tiver certeza se para usar uma matriz ou um vetor, é
 * provável que você use um vetor. Capítulo 8 discute vetores com mais detalhes.
 * No entanto, arrays são mais úteis quando você sabe que o número de elementos não precisa mudar.
 * Por exemplo, se você estiver usando os nomes do mês em um programa, você
 * provavelmente usaria uma matriz em vez de um vetor porque você sabe ele sempre conterá 12 elementos:

 * let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
 * Você escreve um tipo de arrayetts usando colchetes com o tipo de cada elemento um ponto-e-vírgula e, em seguida, o número de elementos na matriz, como assim:
 *  let a: [i32; 5] = [1, 2, 3, 4, 5];
 * Aqui, i32 é o tipo de cada elemento. Depois do ponto e vírgula, o número 5 indica que a matriz contém cinco elementos.
 * Você também pode inicializar uma matriz para conter o mesmo valor para cada elemento por
 * especificando o valor inicial, seguido por um ponto-e-vírgula e, em seguida, o comprimento de a
 * matriz entre colchetes, como mostrado aqui:
 * let a = [3; 5];
 * A matriz nomeada a conterá 5 elementos que serão todos definidos para o valor 3 inicialmente.
 * o mesmo que escrever let a = [3, 3, 3, 3, 3]; mas em um maneira mais concisa.
 *
 * Acessando Elementos de Matriz
 * - Uma matriz é um único pedaço de memória de um tamanho fixo conhecido que pode ser alocado
 * - na pilha. Você pode acessar elementos de um array usando indexação assim:
 * - let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
 * Neste exemplo, a variável nomeada first vai receber o valor 1 porque isso é o valor no índice
 * [0] na matriz. A variável nomeada second vai conseguir o valor 2 do índice [1] na matriz.
 *
 *
 * Acesso Invalido ao Elemento Array
 * Letilits ver o que acontece se você tentar acessar um elemento de uma matriz que é passado o fim
 * da matriz. Digamos que você execute este código, semelhante ao jogo de adivinhação em Capítulo 2,
 * para obter um índice de matriz do usuário:
 use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
* Este código compila com sucesso. Se você executar este código usando cargo run e entrar 0 1 2
* 3, ou 4, o programa imprimirá o correspondente valor nesse índice na matriz. Se você inserir um
* número após o final de a matriz, como 10, youilitll ver saída como esta:
 * thread 'main' panicked at src/main.rs:19:19:
  index out of bounds: the len is 5 but the index is 10
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
* O programa resultou em um tempo de execução erro no ponto de usar um inválido valor na
* operação de indexação. O programa saiu com uma mensagem de erro e didelitt executar a final println! declaração.
* Quando você tenta acessar um elemento usando indexação, Rust irá verificar se o índice especificado é menor do que o comprimento da matriz.
* Se o índice for maior ou igual ao comprimento, A ferrugem entrará em pânico.
* Esta verificação tem que acontecer em tempo de execução, especialmente neste caso porque o compilador pode descobrir qual o valor
* que um usuário irá inserir quando execute o código mais tarde.
* Este é um exemplo dos princípios de segurança de memória Rustilitis em ação.
* Em muitos linguagens de baixo nível, esse tipo de verificação não é feito e quando você fornece um índice incorreto, memória inválida pode ser acessada.
* A ferrugem protege-te contra isto tipo de erro saindo imediatamente em vez de permitir o acesso à memória e continuando.
* O Capítulo 9 discute mais sobre o tratamento de erros do Rustetts e como você pode escreva código legível e seguro que não entre em pânico nem permita acesso inválido à memória.
* 
*/




}
