# Principios SOLID
Trabalho de Engenharia de Software (BCC3004). Os princípios escolhidos foram "Interface Segregation Principle"
(Princípio da Segregação de Interfaces), "Dependency Inversion Principle" (Princípio da Inversão de Dependências)
e "Liskov Substitution Principle" (Princípio da Substituição de Liskov). Além desses, também serão abordados a
Lei de Deméter e Composição Sobre Herança.

A ordem citada acima não é a ordem em que os princípios serão explicados. Eles estão explicados na ordem mais
conveniente para exemplificá-los, de modo a relacionar todos eles.

Atente-se a todos os comentários presentes nos códigos: são explicações tal qual este markdown.

## Dependency Inversion
Este princípio é satisfeito quando, ao invés de instânciar classes diretamente, é preferível receber instâncias destas
como parâmetros da função que as utilizam.

Vamos considerar uma função `buscar_samambaias`, que busca uma lista de samambaias em algum banco de dados
fictício:
- Veja o contra-exemplo da função: [`buscar_samambaias`](./exemplos/src/donts/dependency_inversion.rs);
- Em seguida, veja o exemplo da mesma função refatorada para satisfazer o princípio:
    [`buscar_samambaias`](./exemplos/src/dos/dependency_inversion.rs)

## Lei de Deméter
A lei de Deméter sugere que evitemos utilizar caminhos muito longo para chegar em um destino dentro de uma função,
de modo a evitar que as funções precisem ter conhecimento demais sobre a estrutura de um objeto.

Ainda na função `buscar_samambaias`, observe:
- O [contra-exemplo](./exemplos/src/donts/demeter.rs);
- A [resolução](./exemplos/src/dos/demeter.rs).

## Composição Sobre Herança
Em Rust, sequer é possível utilizar heranças (fa-se notória a força da crença que composição é preferível).
A verdade é que tudo que é escrito como herança pode ser representado como composição.

Fazer essa troca permite maior transparência no código: ao invés de acessar métodos da classe base como se fosse
seu e acabar ficando confuso por poder acessar métodos que saem do "nada", com composição você acessa diretamente
a origem e ainda mantém um bom encapsulamento.

Além do mais, na hora de utilizar mais de 1 classe base, é incomparável a suavidez com que podemos fazê-lo com
composições com os horrorosos *mixins*.

Saindo um pouco da estética e fluidez do código, herança acaba tornando o código mais acoplado. Mudanças na classe
base acabam impactando diretamente a classe que a herda, e todos os métodos públicos da classe base são diretamente
acessíveis pela classe filha, o que também pode ferir o encapsulamento.

Vamos considerar a seguinte situação:
```ts
import {Comestível, Bebível, Dirigível} from "@/common";

class Pessoa {
    protected _nome: string;
    protected _dataNascimento: Date;

    public nome(): string { return this.nome; }
    public dataNascimento(): Date { return this.dataNascimento; }

    public andar() {}
    public comer<T>(comida: T extends Comestível) {}
    public beber<T>(bebida: T extends Bebível) {}

    // outras ações que pessoas quaisquer podem realizar
    public passear();
    public dirigir<T>(veículo: T extends Dirigível) {}
    // ...
}

class Funcionário extends Pessoa {
    public trabalhar();
}
```

Nesse exemplo, esse código seria válido:
```ts
const funcionário: Funcionário = /* ... */;
✅ funcionário.passear();
✅ funcionário.comer(/* ... */);
✅ funcionário.trabalhar();
✅ console.log(funcionário.nome());
```

Enquanto, de fato, faz sentido que um funcionário possa fazer tudo que uma pessoa faz, afinal, ele *é* uma pessoa,
devemos ser francos e aceitar que, em um software, simulando talvez um ambiente profissional, não é interessante
pra ninguém fazer com que o funcionário passeie ou coma algo. Muito provavelmente, são métodos irrelevantes para o
domínio, porém acessíveis para ele.

Resumindo:
- Interessa ao domínio que o funcionário tenha o método `trabalhar` e que tenha um `nome` e uma `dataNascimento`.
- Não é do interesse do domínio forçar o funcionário comer ou beber qualquer coisa.

Podemos resolver tudo isso trocando a herança por composição:
```ts
import {Comestível, Bebível, Dirigível} from "@/common";

class Pessoa {
    protected _nome: string;
    protected _dataNascimento: Date;

    public nome(): string { return this.nome; }
    public dataNascimento(): Date { return this.dataNascimento; }

    public andar() {}
    public comer<T>(comida: T extends Comestível) {}
    public beber<T>(bebida: T extends Bebível) {}

    // outras ações que pessoas quaisquer podem realizar
    public passear();
    public dirigir<T>(veículo: T extends Dirigível) {}
    // ...
}

class Funcionário {
    private pessoa: Pessoa;

    public trabalhar();

    public nome() {
        return this.pessoa.nome();
    }

    public dataNascimento() {
        return this.pessoa.dataNascimento();
    }

    // outros métodos relevantes para um funcionário, no ponto de vista do nosso domínio fictício
    // poderiam e deveriam ser "sobrescritos" também
}
```

Agora,
```ts
const funcionário: Funcionário = /* ... */;
❌ funcionário.passear();
❌ funcionário.comer(/* ... */);
✅ funcionário.trabalhar();
✅ console.log(funcionário.nome());
```

## Liskov Substitution Principle
Este princípio é satisfeito quando podemos substituir a dependência de uma função por uma especialização dessa
sem que ela perceba. É um caso clássico de polimorfismo e é melhor aproveitada quando a inversão de dependências
já foi aplicada.

Praticamente, dada uma função de assinatura `fn baz(bar: &Bar)`, sendo `Foo` uma classe que herda `Bar`, deveria
ser possível realizar a chamada `baz(&foo)`, onde `foo: Foo`.

Em Rust, porém, não temos classes. Muito menos herança. A verdade é que, mesmo com composição, podemos atender esse
princípio: quer por meio de interfaces, quer por meio de "downcasts".

Em primeiro momento, vamos criar uma nova estrutura:
[SamambaiaDatastoreComComposição](./exemplos/src/common/samambaia_datastore_composta.rs). Ela é uma especialização da já conhecida `Datastore`, porém com alguns métodos utilitários.

A função `buscar_samambaias` espera uma instância de `Datastore`, não da nossa especialização. Porém, se alternarmos
a assinatura para receber uma referência de `Datastore`, podemos brincar com um polimorfismo super básico e passar uma
referência da `SamambaiaDatastoreComComposição` no lugar.

Para atingir isso, em Rust, basta implementarmos a trait `AsRef<Datastore>` para nossa classe:

- Veja a [nova assinatura e a chamada da função](./exemplos/src/common/implementando_as_ref_para_datastore_composta.rs).

Outro meio para satisfazer este princípio, seria utilizar interfaces (em Rust, traits). Nesse caso, esperamos
não a interface de uma classe base, mas sim um objeto qualquer que satisfaça a interface (abstrata) esperada.

Para dar esse exemplo, porém, vamos aproveitar e começar a falar do próximo princípio:
