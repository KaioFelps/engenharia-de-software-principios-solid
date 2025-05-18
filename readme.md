# Principios SOLID
Trabalho de Engenharia de Software (BCC3004). Os princípios escolhidos foram "Single Responsibility Principle"
(Princípio da Responsabilidade Única), "Dependency Inversion Principle" (Princípio da Inversão de Dependências)
e "Liskov Substitution Principle" (Princípio da Substituição de Liskov). Além desses, também serão abordados a
Lei de Deméter e Composição Sobre Herança.

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

<!-- 
## Single Responsibility Principle
Este princípio diz que um método/classe/função deve ter somente uma responsabilidade.

Ironicamente, mesmo após a separação, a composição do método ainda consiste em executar várias coisas, porém
cada etapa está encapsulada na sua própria função (que pode ser reutilizada mais tarde). Isso quer dizer que
podemos satisfazer esse requesito por meio de refatorações.

Confira a função ` -->