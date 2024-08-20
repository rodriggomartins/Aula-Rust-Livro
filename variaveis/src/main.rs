fn main(){

  /* As variaveis em rust sao imutaveis, entao elas nao pode receber um valor
   * isso e para evitar problemas, mas caso eu queira mudar, eu tenho que colocar
   * um *** mut ***
   * com o mut que nos podemos fazer com que o x que tava com o valor 5
   * ele ira poder receber outro valor
   * lembrando que por padrao o tipo das variaveis em rust sao do tipo i32
   */ 
    
    let mut x = 5;
    println!("O valor e: {}", x);

    // tentando mudar o valor de x
    x = 6;

    println!("O valor de x agora e: {x}");

    // variaveis e constants
    // const ou constante e uma variavel que nao pode mudar o valor
    // e nao se pode usar o mut em uma constante
    // e tenho que colocar o tipo da constante
    // uma constante pode ser declarada em qualquer parte do codigo, tanto local
    // dentro da fn ou global, fora da fn, mas posso tmb colocar um const dentro da fn
    // A última diferença é que as constantes podem ser definidas apenas para uma expressão constante, ou seja, não pode ser o resultado de uma chamada de função ou qualquer outro valor que só poderia ser calculado em tempo de execução.

    //Aqui está um exemplo de uma declaração constante, em que o nome da constante é
    //PONTOS_MAXIMOS e o valor definido é 100,000 (por convenção, contantes em Rust são nomeadas usando maiúsculas e sublinhado entre as palavras):

    // GERALMENTE, CONST SAO DECLARADAS ANTES DA fn main(){}



   //const PONTOS_MAXIMOS: u32 = 100_000;

   const PONTOS_MAXIMOS: u32 = 100_000;
   println!("const {}", PONTOS_MAXIMOS);


   /*
    * SHADOWING
    *
    *  Como você viu no tutorial do jogo de adivinhação em Capítulo 2, é possível declarar um nova variável com o mesmo nome de uma variável anterior. Os Rustáceos dizem que o a primeira variável é sombreado pelo segundo, o que significa que o segundo variável é o que o compilador verá quando você usar o nome da variável. Com efeito, a segunda variável ofusca a primeira, tomando quaisquer usos do nome da variável para si mesmo até que seja sombreado ou o escopo termine. Podemos sombrear uma variável usando o mesmo nome de variáveis e repetindo o uso do let palavra-chave da seguinte forma:
    *
    *  Este programa primeiro se liga x para um valor de 5. Em seguida, cria uma nova variável x repetindo let x =, tomando o valor original e adicionando 1 então o valor de x é então 6. Então, dentro de um escopo interno criado com o encaracolado colchetes, o terceiro let declaração também sombras x e cria um novo variável, multiplicando o valor anterior por 2 dar x um valor de 12. Quando esse escopo termina, o sombreamento interno termina e x retorna ao ser 6. Quando executamos este programa, ele produzirá o seguinte:


    * 
    */

   let y = 5;
   let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    /*
     * A sombra é diferente de marcar uma variável como mut porque nós vamos conseguir um erro de tempo de compilação se acidentalmente tentarmos reatribuir a essa variável sem usando o let palavra-chave. Usando let, é possível realizar algumas transformações em um valor, mas a variável seja imutável após essas transformações terem sido foi concluído.

A outra diferença entre mut e sombreamento é isso porque weilitre efetivamente criando uma nova variável quando usamos o let palavra-chave novamente, podemos altere o tipo do valor, mas reutilize o mesmo nome. Por exemplo, digamos que o nosso programa pede a um usuário para mostrar quantos espaços eles querem entre algum texto por inserindo caracteres de espaço e, em seguida, queremos armazenar essa entrada como um número:
    */

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");
     

    /* O primeiro spaces variável é um tipo de string e o segundo spaces variável é um tipo de
     * número. A sombra, portanto, nos poupa de ter que inventar nomes diferentes, como spaces_str
     * e spaces_num; em vez disso, podemos reutilizar o mais simples spaces nome. No entanto, se
     * tentarmos usar mut para isso, como mostrado aqui, weletull obter um erro de compilação:*/


    /* O primeiro spaces variável é um tipo de string e o segundo spaces variável é um tipo de número. A sombra, portanto, nos poupa de ter que inventar nomes diferentes, como spaces_str e spaces_num; em vez disso, podemos reutilizar o mais simples spaces nome. No entanto, se tentarmos usar mut para isso, como mostrado aqui, weletull obter um erro de compilação:
*/
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("{spaces}");

}
