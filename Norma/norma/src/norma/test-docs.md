# Inserção

## Classes de valores testadas
- Registrador com natural maior que zero.
- Registrador com natural igual a zero.

## Propriedades testadas
- Inserir um registrador zerado e em seguida obtê-lo resulta no valor zero.
- Inserir um registrador não-zerado e obtê-lo resulta no valor inicial.
- Inserir registrador não altera o valor dos demais.

# Obtenção do conteúdo de registradores inválidos

## Classes de valores testadas
- Registrador inexistente.

## Propriedades testadas
- Buscar um registrador inexistente resulta em erro de programação irrecuperável
    "panic".

# Incremento de registradores

## Classes de valores testadas
- Registrador com natural maior que zero.
- Registrador com natural igual a zero.

## Propriedades testadas
- Incrementar um registrador zerado resulta em `1`.
- Incrementar um registrador não-zerado (`= n`) resulta em `n + 1`.
- Incrementar um registrador não altera o conteúdo dos demais.

# Decremento de registradores

## Classes de valores testadas
- Registrador com natural maior que zero.
- Registrador com natural igual a zero.

## Propriedades testadas
- Decrementar um registrador zerado resulta em `0`.
- Decrementar um registrador não-zerado (`= n`) resulta em `n - 1`.
- Decrementar um registrador não altera o conteúdo dos demais.

# Teste "is zero" de registradores

## Classes de valores testadas
- Registrador com natural maior que zero.
- Registrador com natural igual a zero.

## Propriedades testadas
- Um registrador zerado resulta positivo para o teste "is zero".
- Um registrador não-zerado resulta negativo para o teste "is zero".
- Manipular o registrador zerado através de métodos como `.inc()` faz com que o
    teste negative.
- Manipular o registrador zerado através de métodos como `.dec()` não altera o
    teste.
- Manipular o registrador não-zerado através de métodos como `.inc()` não altera
    o teste.
- Manipular o registrador não-zerado através de métodos como `.dec()` faz com
    que o teste positive.

# Adição de uma constante a um registrador

## Classes de valores testadas
- Registrador com natural maior que zero com quantidade não-zerada.
- Registrador com natural maior que zero com quantidade nula.
- Registrador com natural igual a zero com quantidade não-zerada.
- Registrador com natural igual a zero com quantidade nula.

## Propriedades testadas
- Adicionar quantidade não-zerada a um registrador não-zerado soma os dois
    corretamente.
- Adicionar quantidade não-zerada a um registrador zerado resulta na quantidade.
- Adicionar quantidade nula a um registrador não-zerado não muda nada.
- Adicionar quantidade nula a um registrador zerado não muda nada.

# Subtração de uma constante de um registrador

## Classes de valores testadas
- Registrador com natural maior que zero com quantidade não-zerada sem saturar.
- Registrador com natural maior que zero com quantidade não-zerada saturando.
- Registrador com natural maior que zero com quantidade nula.
- Registrador com natural igual a zero com quantidade não-zerada.
- Registrador com natural igual a zero com quantidade nula.

## Propriedades testadas
- Subtrair quantidade não-zerada de um registrador não-zerado, tal que a
    quantidade cabe no registrador, subtrai os dois corretamente, sem saturar.
- Subtrair quantidade não-zerada de um registrador não-zerado, tal que a
    quantidade não cabe no registrador, subtrai os dois saturando.
- Subtrair quantidade não-zerada a um registrador zerado resulta em zero.
- Subtrair quantidade nula a um registrador não-zerado não muda nada.
- Subtrair quantidade nula a um registrador zerado não muda nada.

# Comparação de igualdade entre uma constante e um registrador

## Classes de valores testadas
- Registrador zerado comparado com zero.
- Registrador não-zerado comparado com valor correspondente.
- Registrador zerado comparado com valor não-zerado.
- Registrador não-zerado comparado com valor não-correspondente.

## Propriedades testadas
- Registrador zerado comparado com zero testa positivo.
- Registrador não-zerado comparado com valor correspondente testa positivo.
- Registrador zerado comparado com valor não-zerado testa negativo.
- Registrador não-zerado comparado com valor não-correspondente testa negativo.

# Obtenção do contador

## Classes de valores testadas
- Estado inicial da mãquina
- Estado da máquina após uma operação simples (como `inc` e `dec`).
- Estado da máquina após operação complexa (como adição com constante).

## Propriedades testadas
- No estado inicial, o contador contém `0`.
- Após cada operação simples, o contador incrementa `1`.
- Após adição ou subtração por constante, o contador incrementa pela constante.
- No caso da comparação com constante, isso está em aberto: precisamos definir a
    expansão da comparação em termos de operações simples.
