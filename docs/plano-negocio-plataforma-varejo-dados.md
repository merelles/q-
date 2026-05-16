# Plano de negocio - Plataforma de varejo, dados fiscais e trade spend

Data base: 2026-05-07  
Versao: 0.2 executiva ampliada
Status: plano de validacao para decisao de fundador

## 1. Tese

A empresa nao deve nascer como mais um marketplace de supermercado. Esse mercado ja tem players com distribuicao, marca, capital e operacao logistica.

A tese defensavel e construir uma infraestrutura de dados e transacao para o varejo alimentar regional:

- capturar dados de compra via leitura de nota fiscal;
- unificar cadastro de produtos por EAN/GTIN;
- conectar consumidores, lojas, distribuidores e industria;
- vender inteligencia comercial e ativacao dentro da jornada de compra;
- permitir venda dentro da plataforma;
- operar carteira, pontos/tokens e cupons para redistribuir parte do fundo de incentivo ao consumidor;
- oferecer servicos B2B de cadastro fiscal, estoque, integracao, venda e delivery.

O ativo principal nao e o app. O ativo principal e a base transacional normalizada, consentida, georreferenciada e acionavel.

## 2. Proposta de valor

### Para consumidores

- Ganhar tokens/pontos ao escanear notas fiscais.
- Trocar tokens por saldo promocional uma vez por mes, conforme regra central do fundo de incentivo.
- Pesquisar precos, produtos, lojas proximas e historico de consumo com limites gratuitos.
- Pagar por mais consultas usando saldo fiat, tokens recebidos ou pacote de tokens.
- Comprar produtos dentro da plataforma com ofertas subsidiadas por lojas, distribuidores ou industria.

### Para supermercados e lojas

- Entrar em um canal de venda digital regional sem depender apenas de iFood, Rappi, Mercado Livre ou e-commerce proprio.
- Receber integracao de estoque, precos, cadastro de produto e pedidos.
- Emitir nota fiscal e organizar delivery com operacao propria ou pela plataforma.
- Corrigir cadastro fiscal/produto e reduzir risco tributario.
- Ter inteligencia de mercado a partir de dados agregados e anonimizados.
- Participar da captura de valor por meio de cotas estrategicas nas rodadas regionais.

### Para industria e distribuidores

- Enxergar demanda real por regiao, loja, categoria e periodo.
- Comprar exposicao contextual na busca e no carrinho.
- Subsidiar descontos seletivos no ponto de decisao do consumidor.
- Impulsionar venda no varejo independente sem depender apenas de grandes redes.

### Para a propria plataforma

- Receita por venda dentro da plataforma.
- Receita por inteligencia de mercado e precificacao.
- Receita por trade spend/campanhas patrocinadas.
- Receita por SaaS e integracoes.
- Receita por revisao fiscal e melhoria cadastral.
- Receita financeira/operacional futura se a carteira evoluir para conta ou arranjo regulado.

## 3. Problema real

O varejo alimentar regional tem cinco dores estruturais:

- Cadastro de produtos ruim, duplicado e fiscalmente inconsistente.
- Baixa maturidade digital nas lojas medias.
- Dependencia de canais com taxas altas e pouco controle de dados.
- Pouca inteligencia sobre demanda local fora do proprio PDV.
- Trade spend da industria mal conectado ao consumidor final.

O consumidor, por outro lado, tem pouco incentivo para compartilhar dados de compra. O token/ponto tenta resolver esse problema, mas so funciona se o valor percebido for simples, recorrente e confiavel.

## 4. Mercado e contexto competitivo

### Tamanho do mercado

O varejo alimentar brasileiro e grande o suficiente para comportar uma infraestrutura regional. O Ranking ABRAS 2025 reportou faturamento do canal alimentar de R$ 1,067 trilhao em 2024, considerando supermercados, atacarejos, minimercados, conveniencia, hortifruti e e-commerce.

Referencia: https://gironews.com/canais-trade/supermercado/ranking-abras-2025-destaca-supermercados-como-forca-motriz-da-economia-nacional/

### Concorrentes diretos e indiretos

#### iFood

Concorrente forte em delivery, supermercados, farmacia, relacionamento com consumidor e eventual oferta financeira. O iFood tem distribuicao, marca, recorrencia e capital. Segundo reportagem recente, deveu encerrar 2025 com cerca de 1,7 bilhao de pedidos no Brasil; outra fonte de mercado indicou lideranca relevante em delivery e crescimento em supermercado e servicos financeiros.

Referencias:

- https://veja.abril.com.br/economia/ifood-batera-recorde-de-pedidos-no-brasil-mesmo-com-mais-concorrentes/
- https://www.moneytimes.com.br/ifood-domina-delivery-com-70-do-mercado-e-enfrenta-nova-concorrencia-de-entregas-rapidas-ceci/

Risco: se a proposta for "app para comprar mercado", perde por CAC, marca, sortimento e logistica.

Oportunidade: iFood e forte na demanda final. A plataforma pode entrar pelo lado B2B regional, dados fiscais, cadastro, estoque, trade spend e coalizao de lojas.

#### Méliuz

Concorrente relevante na camada de nota fiscal, cashback e ativacao de consumidor. A empresa usa nota fiscal como canal de campanha, recompensa e relacionamento, com patrocinio de marcas parceiras e regras por campanha.

Risco: se a tese ficar restrita a "escaneie nota e receba cashback", Méliuz e outros players de incentivo podem copiar o formato rapido.

Oportunidade: diferenciar pela integracao com loja, catalogo EAN, trade spend mensuravel, dados agregados e venda incremental regional.

Referencias:

- https://www.meliuz.com.br/blog/meliuz-nota-fiscal/
- https://www.meliuz.com.br/como-funciona
- https://ajuda.meliuz.com.br/hc/pt-br/articles/12289793401236-Como-envio-minha-nota-fiscal

#### Mercado Livre

Concorrente em e-commerce, pagamento, logistica, publicidade e eventualmente farmacia/mercado. Tem base de usuarios e infraestrutura dificil de replicar.

Risco: competir em marketplace horizontal e guerra perdida.

Oportunidade: focar no varejo local, nota fiscal alimentar e inteligencia de categoria.

#### Rappi, Daki, Shopper e apps regionais

Atuam em conveniencia, supermercado, entrega rapida ou compra programada. O risco esta em experiencia de compra e entrega. A oportunidade esta em nao depender de estoque proprio no inicio.

#### Grandes redes supermercadistas

Carrefour, Assai, Mateus, BH, GPA e redes regionais relevantes ja possuem dados, escala e fornecedor. Nao devem ser o primeiro alvo.

Oportunidade: lojas medias com faturamento mensal relevante, mas sem capacidade tecnica para montar plataforma propria.

#### Software houses/ERPs de varejo

Sao concorrentes invisiveis. Eles ja estao conectados ao PDV, estoque, fiscal e cadastro. Podem bloquear integracao, copiar funcionalidades ou virar canal.

Estrategia recomendada: tratar ERP como parceiro quando possivel e construir conectores padrao quando necessario.

#### Empresas de dados e inteligencia de mercado

NielsenIQ, Scanntech, Neogrid, Horus e similares disputam inteligencia de sell-out, categoria e trade marketing.

Risco: a proposta de vender inteligencia de mercado nao e nova.

Oportunidade: a diferenca precisa estar em dado granular, regional, acionavel dentro da venda, com loop de incentivo ao consumidor e lojista.

#### Involves, InfoPrice e ferramentas de execucao/precificacao

Involves atua em trade marketing e execucao de campo; Neogrid e Horus atuam em sell-out, estoque, preco e inteligencia de mercado; soluções como InfoPrice e similares atacam monitoramento de preco e competitividade. Sao concorrentes indiretos porque ocupam parte da mesma cadeia de valor.

Risco: cada uma dessas empresas pode capturar um pedaço do problema antes de voce.

Oportunidade: vencer pela unificacao do fluxo consumidor-loja-industria dentro de um unico loop transacional.

#### Memed

Memed e referencia de prescricao digital, nao concorrente central neste pivot de varejo alimentar. O aprendizado relevante e que infraestrutura regulada/cadastral pode virar ativo valioso, mas varejo fisico e logistica exigem escala pesada.

Referencia: https://memed.com.br/medicos/

### Matriz de concorrentes

| Concorrente | Fraqueza dele | Como ele te engole | Como impedir isso |
|---|---|---|---|
| Méliuz | Depende de cashback e campanha; se o incentivo cair, a adesao cai. | Captura o gesto inicial de escanear nota e vira o padrao mental do consumidor. | Entrar por utilidade superior: loja, preco, EAN, compra, trade spend e valor recorrente. |
| Scanntech | Forte em inteligência, mas menos centrada no consumidor final e no incentivo de comportamento. | Toma a camada de inteligencia acionavel e vira o fornecedor default da industria. | Criar dado consentido do consumidor + ativacao em jornada real + medicao de incrementalidade. |
| Neogrid | Ecossistema amplo, mas mais orientado a dados de sell-out, estoque e colaboração entre grandes players. | Ocupa a conversa com industria e varejo antes de voce e define a agenda. | Focar em varejo regional, ofertas contextuais e loop transacional com recompensa. |
| Horus | A proposta é forte em nota fiscal e shopper, mas depende muito da qualidade do dado capturado. | Usa a nota fiscal como fonte de dados e fica na frente do seu core de captura. | Ir alem da captura: integrar loja, catalogo, precificacao e conversao em compra. |
| Involves | Muito forte em trade marketing e execucao de campo, mas nao resolve o gesto do consumidor. | Entra pelo budget da industria e fecha a porta da ativacao no PDV. | Ser a ponte entre industria, loja e consumidor, nao apenas a ferramenta da industria. |
| InfoPrice | Forte em monitoramento de preco, mas o valor fica no comparativo, nao na transacao. | Vira a referencia de preco e tira sua vantagem de precificacao. | Trabalhar com preco confirmado por venda efetiva, contexto e historico proprio. |
| iFood | Marca forte, logistica, recorrencia e capacidade de comprar crescimento. | Pode dominar a ultima milha e o ponto final da compra. | Nao competir no front de marketplace horizontal; ganhar pelo B2B regional e dados fiscais. |
| Mercado Livre | Escala, logistica, publicidade e pagamentos com poder de empurrar o consumidor para dentro. | Verticaliza e horizontaliza ao mesmo tempo, comprimindo a sua margem de espaco. | Evitar guerra de marketplace e focar no varejo alimentar local e fiscal. |
| ERPs e software houses | Controlam a entrada tecnica do lojista e podem bloquear, copiar ou condicionar integracao. | Se eles travam a integracao, voce perde o acesso ao dado e ao fluxo. | Ser parceiro padronizavel, com conectores simples e proposta clara de ganho para o ERP. |

## 5. Produto

### Modulos principais

#### App de validade e inventario

- Leitura de EAN e nota fiscal para registrar entrada de estoque.
- Controle de validade por lote, com foco inicial em laticinios e pao de forma.
- Alertas de proximidade de vencimento para reducao de perda e ruptura.
- Inventario simples por loja, com rastreio de produto, quantidade e data de entrada.
- Base operacional para validar aderencia do lojista antes do rollout do core completo.

#### App consumidor

- Leitura de QR Code da NFC-e ou upload/foto da nota.
- Extracao e normalizacao de itens.
- Pontuacao/token por nota validada.
- Carteira de saldo, tokens e cupons.
- Busca de produtos, lojas e precos por localizacao.
- Compra na plataforma.
- Historico de consumo.
- Limites de consulta gratuitos e planos/pacotes pagos.

#### Backoffice lojista

- Cadastro de loja, usuarios, areas de entrega e politicas comerciais.
- Cadastro de produtos por tenant.
- Integracao de estoque e preco.
- Pedidos, separacao, pagamento, emissao fiscal e status de entrega.
- Gestao de campanhas e cupons.
- Relatorios de sell-out e comparativos agregados.

#### Plataforma de dados

- Tabela global de EAN/GTIN sem nome comercial oficial como verdade absoluta.
- Nome, descricao, categoria, imagem, fiscal e aliases por tenant.
- TENANT_DEFAULT validado para padronizacao negociada e melhoria de cadastro de clientes.
- Data warehouse para notas, itens, consumidores, lojas, produtos, categorias, precos e campanhas.
- Consulta complementar de precos em fontes publicas de SEFAZ, quando a estrategia exigir e a UF permitir, como Parana, Bahia, Ceara e outros estados com disponibilidade tecnica.
- Registro de melhor oferta confirmada por venda efetiva, permitindo comparar lojas que cobrem ofertas sem tratar preco apenas consultado como verdade comercial.
- Camada de consentimento, anonimização/pseudonimizacao e governanca LGPD.

#### Motor de trade spend

- Campanhas patrocinadas por industria, distribuidor ou loja.
- Oferta contextual: exemplo, busca por "chocolate Garoto" pode exibir produto concorrente ou complementar com desconto subsidiado.
- Regras por localizacao, estoque, categoria, margem, periodo e perfil de compra.
- Medicao de conversao, incrementalidade e ROI.

#### Core promocional preventivo

- Publicacao de promocoes com foco inicial em prevencao de perda, giro e vencimento.
- Destaque para itens com validade curta, excesso de estoque ou risco de ruptura.
- Regras simples de priorizacao para testar adesao do lojista e resposta do consumidor.
- Escopo limitado no inicio: nao e motor completo de trade spend, e sim a camada base de promocao do MVP.

#### Fiscal e compliance

- Emissao de NFC-e/NF-e para vendas feitas dentro da plataforma.
- Integracao fiscal com sistemas dos vendedores.
- Servico de revisao tributaria do cadastro de produtos.
- Atualizacao cadastral de pessoas juridicas via dados oficiais da Receita Federal.

Referencias oficiais:

- CNPJ Receita Federal: https://www.gov.br/receitafederal/pt-br/servicos/cadastro/cnpj
- Consulta CNPJ: https://www.gov.br/pt-br/servicos/consultar-cadastro-nacional-de-pessoas-juridicas

#### Entrega

- Modo vendedor: loja gerencia separacao e entrega.
- Modo parceiro: parceiros logisticos homologados realizam a entrega com SLA, rastreio e regras operacionais definidas pela plataforma.
- Modo plataforma: plataforma gerencia entregador, SLA e rastreio.
- Modo hibrido: loja separa, plataforma entrega.

Recomendacao: comecar com modo vendedor, modo parceiro e modo hibrido. Logistica propria no dia 1 aumenta capital, risco operacional e complexidade.

## 6. Modelo de dados conceitual

### Entidades centrais

- Tenant
- Pessoa
- Pessoa juridica
- Pessoa fisica
- Endereco georreferenciado
- Loja
- Produto global
- Produto tenant
- EAN/GTIN
- Categoria
- NCM/CEST/tributacao
- Estoque
- Preco
- Nota fiscal escaneada
- Item da nota
- Pedido
- Item do pedido
- Campanha
- Cupom
- Wallet
- Token/ponto
- Lancamento financeiro
- Consentimento
- Evento de auditoria

### Decisao importante sobre EAN

A ideia de "EAN sem nome e nome por tenant" e tecnicamente boa para multi-tenant porque evita impor uma verdade unica ruim. Mas precisa de um TENANT_DEFAULT muito bem governado.

Risco: se cada tenant chamar o mesmo EAN de forma muito diferente, a base fica rica para operacao local, mas fraca para analytics global.

Contramedida:

- Produto global guarda identificadores e atributos normalizados.
- Produto tenant guarda descricao comercial local.
- Alias engine agrupa nomes equivalentes.
- TENANT_DEFAULT vira camada curada e vendavel.
- Alteracao no TENANT_DEFAULT exige workflow, origem, score de confianca e auditoria.

## 7. Modelo de negocio

### Receita B2B

- Setup de integracao por loja.
- Mensalidade SaaS por loja.
- Percentual sobre venda dentro da plataforma.
- Revisao fiscal de cadastro de produtos.
- Enriquecimento cadastral baseado no TENANT_DEFAULT.
- Analytics regional e comparativo.
- API para consulta de produto, preco, estoque e demanda.

### Receita B2C

- Pacotes de tokens para consulta.
- Assinatura premium para remover limites.
- Spread/taxa operacional, se juridicamente permitido, em funcionalidades financeiras futuras.

### Receita B2B2C

- Campanhas patrocinadas.
- Cupons subsidiados.
- Destaque em busca.
- Recomendacao contextual.
- Promocao por categoria, regiao, loja ou perfil agregado.

### Fundo de incentivo e retencao

O fundo de incentivo nao e take rate da plataforma. Ele e uma retencao variavel sobre as vendas feitas dentro da plataforma, calculada por item vendido conforme a curva de rotatividade do produto:

- Curva A: 5% do valor vendido do item.
- Curva B: 10% do valor vendido do item.
- Curva C: 15% do valor vendido do item.

A curva deve ser definida por criterio objetivo de giro, margem, disponibilidade e estrategia comercial. Produto de maior rotatividade exige menor retencao. Produto de menor rotatividade pode sustentar maior incentivo, desde que a margem e o acordo comercial permitam.

Na media, uma retencao proxima de 10% sobre o valor vendido nao deve ser lida como taxa excessiva se o mecanismo provar recompra, recorrencia e venda incremental. Parte relevante desse valor volta para o proprio ecossistema: o consumidor recebe incentivo para comprar novamente, a loja aumenta chance de recompra e a plataforma gera inteligencia de mercado e precificacao para melhorar ativacoes futuras.

Formula do fundo no periodo:

```text
Fundo do periodo = soma(valor vendido do item x percentual da curva do item)
```

Distribuicao mensal do fundo:

- 20% do fundo sera convertido em token/saldo de leitura de notas fiscais e reinvestido em mecanismos para o cliente continuar consumindo na plataforma.
- 60% do fundo sera convertido em cupom de desconto para compras dentro da plataforma.
- 20% do fundo ficara como reserva operacional do proprio mecanismo, para antifraude, chargeback, arredondamentos, ajustes de campanha e contingencias.

A parcela de incentivo ao consumidor tambem alimenta inteligencia de mercado acionavel, porque aumenta recorrencia, leitura de notas, historico de consumo e resposta a campanhas. Isso permite ativacoes como notificacoes de promocoes relampago, recomendacoes por categoria, alertas de preco, cupons contextuais e outras mecanicas comerciais que ainda serao descobertas nos testes.

Formula da distribuicao para leitura de notas:

```text
Saldo de leitura do usuario = parcela de leitura do fundo x (tokens do usuario no periodo / total de tokens emitidos no periodo)
```

Essa regra deve ser tratada como politica de incentivo e liquidez, nao como receita bruta da empresa.

### O que nao cobrar no inicio

Nao cobrar pesado do consumidor antes de provar valor. O consumidor ja precisa ter trabalho de escanear nota. Se o premio for baixo e a consulta for paga cedo demais, o funil morre.

## 8. Estrategia de kickoff

### Teste antes do kickoff

Antes de abrir o kickoff completo da plataforma, executar um teste curto com um objetivo operacional unico:

- criar uma app de validade que leia EAN e nota fiscal para registrar inventario;
- controlar validade de laticinios e pao de forma;
- reduzir perda por vencimento e melhorar visibilidade de estoque;
- montar, em paralelo, o core da plataforma para publicar promocoes preventivas;
- manter o escopo promocional restrito a prevencao neste momento, sem tentar fechar o produto final cedo demais.

Critério do teste:

- o lojista precisa conseguir registrar entrada, saldo e vencimento com friccao baixa;
- o usuario interno precisa enxergar valor em perda evitada, nao em promessa abstrata;
- a base precisa provar que leitura de nota e EAN gera dado util para operacao e promocao.

### Hipotese inicial

Selecionar 10 lojas com faturamento mensal aproximado de R$ 3 milhoes cada.

Cada loja selecionada entra em um periodo de prova de 12 meses. A conversao em cotas nao acontece pela simples assinatura ou integracao; acontece apenas se a loja comprovar entrega das metas pactuadas no periodo.

Objetivo:

- converter 10% das vendas dessas lojas para dentro da plataforma;
- aplicar a regra central do fundo de incentivo sobre as vendas transacionadas;
- usar as integracoes como base para o produto;
- vender revisao fiscal/cadastral;
- usar consumidores dessas lojas para alimentar a base de notas e precos;
- provar que campanhas patrocinadas aumentam sell-out.

### Numeros da hipotese

Faturamento total das 10 lojas:

- R$ 30 milhoes/mes.

Meta de conversao para plataforma:

- 10% = R$ 3 milhoes/mes em GMV.

Fundo de incentivo:

- calculado pela retencao por curva definida na secao de modelo de negocio.

Leitura dura:

- fundo de incentivo nao paga a operacao, porque nao e receita da plataforma.
- Esse kickoff serve para aprendizado, dados, prova comercial e integracao.
- Nao serve como motor financeiro da empresa.

Para a empresa ficar interessante, precisa de pelo menos uma das tres coisas:

- comissao propria da plataforma, separada do fundo de incentivo;
- receita relevante de trade spend;
- SaaS/servicos B2B cobrados fora do percentual de venda.

### Meta minima de validacao em 6 meses

- 10 lojas integradas.
- R$ 3 milhoes/mes de GMV.
- 30 mil a 80 mil consumidores cadastrados.
- 200 mil+ notas escaneadas/mes, se o incentivo funcionar.
- Base com 100 mil+ EANs tratados.
- 3 industrias/distribuidores pagando campanha piloto.
- Receita mensal recorrente acima de R$ 100 mil somando SaaS, servicos, comissao propria e campanhas. O fundo de incentivo nao entra como receita recorrente.

Se a loja bater todas as metas pactuadas ao fim dos 12 meses, as cotas previstas para ela poderao ser convertidas conforme contrato. Se nao bater todas as metas, a situacao deve ser analisada pontualmente, considerando volume entregue, qualidade da integracao, recorrencia operacional, dados gerados, exclusividade regional, aderencia comercial e potencial estrategico.

Se a receita ficar abaixo disso, ha sinal de que o produto e util, mas nao economico.

## 9. Escala regional e cotas

### Rodada regional

Selecionar 100 lojas estrategicas para escala regional.

Proposta:

- distribuir ate 10% de cotas para essas lojas em uma rodada estrategica;
- repetir em ate 4 ondas de 10%, totalizando potencialmente 40%.

Leitura dura:

- Ceder 40% para lojas pode destruir governanca e cap table se nao houver criterio claro.
- Loja nao deve receber equity apenas por aderir. Deve receber por entregar volume, exclusividade regional, integracao, dados, verba comercial ou distribuicao.

Modelo recomendado:

- pool maximo por onda condicionado a metas;
- vesting por performance;
- cliff de 12 meses;
- conversao de cotas somente apos comprovacao das metas do periodo de prova;
- recompra ou redistribuicao se a loja sair;
- direitos economicos sem controle operacional;
- regras contra conflito com concorrentes.

### Saida de loja socio-estrategica

Se uma loja optar por sair do contrato social, a preferencia de compra das cotas sera sempre dos socios existentes.

A opcao de compra deve ser oferecida primeiro de forma rateada entre os socios, proporcionalmente a participacao de cada um. As cotas nao adquiridas nessa primeira rodada poderao ser ofertadas aos socios que manifestarem interesse em comprar acima da propria proporcao.

Se, apos essas rodadas internas, ainda sobrarem cotas sem comprador, elas devem ser incorporadas ao negocio conforme previsao societaria, sem obrigar entrada de terceiros e sem transferir poder operacional a concorrentes ou agentes desalinhados.

## 10. Tokens, carteira e dinheiro

### Funcao inicial recomendada

Comecar como pontos internos de fidelidade, nao como criptoativo negociavel.

Motivo:

- reduz risco regulatorio;
- reduz necessidade de autorizacao financeira;
- simplifica contabilidade;
- evita promessa de investimento;
- deixa claro que o ponto e beneficio promocional, nao ativo financeiro.

### Conversao mensal

O usuario escaneia notas e acumula pontos. Uma vez por mes, a plataforma converte a parcela de leitura de notas do fundo de incentivo em saldo promocional, conforme a regra unica definida no modelo de negocio.

Ponto critico:

- se o usuario nao souber como o saldo e calculado, a proposta perde forca;
- se prometer valor fixo alto, a conta pode quebrar;
- se o valor for baixo demais, ninguem escaneia.

### Carteira e conta

A carteira pode evoluir em etapas:

1. Saldo promocional fechado dentro da plataforma.
2. Saldo fiat para compras dentro da plataforma.
3. Integracao com servico financeiro de terceiro para emitir cartao virtual e permitir uso do saldo fiat fora da carteira da plataforma.
4. Conta regulada via parceiro financeiro, se a tracao e o compliance justificarem.
5. Token negociavel, somente se fizer sentido regulatorio e economico.

O passo 5 nao deve entrar no MVP.

Referencia regulatoria contextual: em 2025 o Banco Central publicou/repercutiu regras para prestadores de servicos de ativos virtuais, aumentando o custo de compliance para quem operar criptoativos.

Referencia: https://agenciabrasil.ebc.com.br/economia/noticia/2025-11/banco-central-estabelece-regras-para-o-mercado-de-criptoativos

## 11. LGPD, inteligencia de mercado e precificacao

Este e um dos pontos mais fracos do plano se for tratado superficialmente.

A empresa nao vai vender dados. O produto vendavel sera inteligencia de mercado e precificacao, construída a partir de informacoes agregadas, anonimizadas e governadas, sem entrega de base pessoal bruta ou dado reidentificavel.

Observacao: qualquer material comercial, contrato ou apresentacao deve evitar a expressao "venda de dados". A linguagem correta e "inteligencia de mercado e precificacao".

Observacao operacional: na leitura de nota ou cupom fiscal, CPF e nome do consumidor nao serao persistidos. Se esses campos aparecerem na origem, devem ser descartados ou mascarados antes da gravacao, mantendo apenas os dados necessarios para validacao da nota, antifraude, pontuacao e inteligencia agregada.

### Riscos

- Nota fiscal pode conter CPF, local, horario, itens comprados e padrao de vida.
- Dados de consumo alimentar podem revelar saude, religiao, renda, rotina familiar e habitos sensiveis por inferencia.
- Geolocalizacao aumenta risco de reidentificacao.
- Industria e varejo vao querer granularidade; LGPD limita o que pode ser compartilhado.
- Anonimizacao mal feita pode ser reversivel.

### Contramedidas obrigatorias

- consentimento claro e granular;
- politica de finalidade;
- minimizacao de dados;
- descarte ou mascaramento de CPF e nome antes da persistencia;
- pseudonimizacao operacional;
- agregacao minima por regiao/categoria/periodo;
- proibicao de exportar base pessoal bruta;
- trilha de auditoria;
- DPO/encarregado;
- DPIA/RIPD para tratamentos de maior risco;
- controles de retencao e exclusao;
- segregacao entre dado identificavel e dado analitico.

A ANPD informa sanções administrativas, incluindo multa de ate 2% do faturamento no Brasil, limitada a R$ 50 milhoes por infracao.

Referencia: https://www.gov.br/anpd/pt-br/canais_atendimento/agente-de-tratamento/perguntas-frequentes-anpd

## 12. Operacao fiscal

### Emissao de nota

Para vendas dentro da plataforma, a responsabilidade fiscal precisa ser desenhada corretamente:

- marketplace puro, vendedor emite nota;
- revenda, plataforma compra e revende, plataforma emite nota;
- intermediacao com split, vendedor emite nota e plataforma emite nota de servico/comissao quando aplicavel;
- full commerce, plataforma assume mais operacao e risco.

Recomendacao:

- comecar como marketplace/intermediador;
- vendedor emite documento fiscal da mercadoria;
- plataforma emite nota de servico/comissao conforme estrutura juridica;
- integracao fiscal deve suportar NFC-e/NF-e por UF.

NFC-e tem mudancas tecnicas em andamento, incluindo nova estrutura de QR Code v3.00 em Nota Tecnica 2025.001.

Referencia: https://inventti.com.br/nf-e-nfc-e-nt-2025-001-publicada-a-versao-1-01-com-nova-estrutura-do-qr-code-3-00-para-nfc-e/

## 13. Unit economics preliminar

### Premissas base

- 10 lojas no piloto.
- R$ 3 milhoes/mes GMV dentro da plataforma.
- Fundo de incentivo calculado por rotatividade, conforme regra unica do modelo de negocio.
- Receita da plataforma separada do fundo de incentivo.
- SaaS por loja: R$ 1.000 a R$ 3.000/mes.
- Receita SaaS: R$ 10 mil a R$ 30 mil/mes.
- Servico fiscal/cadastro: R$ 5 mil a R$ 20 mil por loja em projeto inicial.
- Campanhas patrocinadas piloto: R$ 20 mil a R$ 100 mil/mes.

### Receita mensal possivel no piloto

Cenario fraco:

- SaaS: R$ 10 mil;
- campanhas: R$ 20 mil;
- comissao/servicos sobre transacao: a validar;
- total sem considerar fundo de incentivo: R$ 30 mil/mes mais comissao/servicos sobre transacao.

Cenario bom:

- SaaS: R$ 30 mil;
- campanhas: R$ 100 mil;
- servicos recorrentes: R$ 30 mil;
- comissao/servicos sobre transacao: a validar;
- total sem considerar fundo de incentivo: R$ 160 mil/mes mais comissao/servicos sobre transacao.

Leitura:

- o piloto so faz sentido se provar campanhas e servicos B2B;
- depender do fundo de incentivo como se fosse receita e erro de modelo.

### Break-even simplificado

Se o custo mensal operacional sem desenvolvimento/infra for R$ 150 mil a R$ 300 mil, o break-even exige:

- GMV muito maior;
- comissao propria sobre transacao;
- campanhas recorrentes;
- SaaS real;
- operacao enxuta.

O fundo de incentivo deve aumentar liquidez, retencao e conversao. Ele nao substitui receita de SaaS, campanhas, servicos, comissao propria e analytics.

## 14. Investimento e custos

### Premissa informada

Infraestrutura sera custeada pelos socios fundadores por 1 ano.

Leitura:

- isso ajuda caixa, mas nao elimina custo economico;
- investidor e socio estrategico vao querer ver custo real da infra;
- se o produto depender de processamento pesado de notas, OCR, busca, eventos e DW, a infra cresce com uso.

### Custos principais

- desenvolvimento;
- integracoes com ERPs/PDVs;
- cloud, observabilidade, DW e filas;
- compliance LGPD;
- juridico societario, fiscal e financeiro;
- suporte lojista;
- suporte consumidor;
- operacao de campanhas;
- antifraude;
- conciliacao financeira;
- emissao fiscal;
- atendimento de entregas;
- comercial enterprise/regional.

### Custo de infraestrutura para os 10 primeiros clientes

Premissa: 10 lojas piloto, R$ 3 milhoes/mes de GMV dentro da plataforma, 30 mil a 80 mil consumidores cadastrados, 200 mil+ notas escaneadas/mes e uso inicial de dashboards, busca, wallet de pontos, campanhas e pipeline de dados.

Estimativa mensal de infraestrutura:

- Cloud transacional, banco, cache, storage e rede: R$ 8 mil a R$ 20 mil/mes.
- Filas, processamento assíncrono, jobs de normalizacao e eventos: R$ 3 mil a R$ 10 mil/mes.
- OCR/leitura de nota, validacao de QR Code e enriquecimento de itens: R$ 8 mil a R$ 35 mil/mes, dependendo do volume e do fornecedor.
- DW/lakehouse, BI, consultas analiticas e retencao historica: R$ 5 mil a R$ 20 mil/mes.
- Busca textual/geoespacial de produtos, lojas e precos: R$ 2 mil a R$ 8 mil/mes.
- Observabilidade, logs, auditoria, backup e seguranca: R$ 4 mil a R$ 12 mil/mes.
- Antifraude, monitoramento de duplicidade e controles de abuso: R$ 3 mil a R$ 15 mil/mes.

Total estimado para os 10 primeiros clientes:

- R$ 33 mil a R$ 120 mil/mes.
- R$ 396 mil a R$ 1,44M/ano.

Leitura: se os fundadores custearem a infraestrutura no primeiro ano, esse valor pode nao sair do caixa da empresa, mas deve aparecer como custo economico real. A faixa baixa exige arquitetura enxuta, poucos fornecedores externos e forte controle de processamento. A faixa alta aparece quando OCR, DW, logs e antifraude crescem sem governanca.

### Orcamento de 12 meses - enxuto, mas realista

- Produto/engenharia: R$ 1,2M a R$ 2,4M.
- Comercial e implantacao: R$ 400k a R$ 900k.
- Juridico/compliance/fiscal: R$ 250k a R$ 600k.
- Operacao/suporte: R$ 300k a R$ 800k.
- Marketing/incentivo consumidor: R$ 300k a R$ 1M.
- Infra/dados: custeada pelos fundadores, mas provisionar R$ 200k a R$ 600k como custo economico.

Total economico de 12 meses:

- R$ 2,65M a R$ 6,3M.

Se tentar incluir carteira financeira robusta, delivery proprio e token negociavel no primeiro ano, o custo e o risco sobem de forma nao linear.

## 15. Roadmap recomendado

### Fase 0 - Validacao juridica e comercial (0 a 60 dias)

- Parecer LGPD sobre nota fiscal, consentimento, compartilhamento e monetizacao.
- Parecer regulatorio sobre pontos, wallet e conversao em dinheiro.
- Validacao fiscal do modelo de intermediacao e emissao de notas.
- Conversas com 20 lojas; selecionar 10.
- Conversas com 10 industrias/distribuidores; fechar 3 pilotos pagos.
- Mapear ERPs usados pelas lojas-alvo.

Gate de decisao:

- sem industria/distribuidor pagando campanha piloto, a tese de trade spend esta fraca;
- sem lojas aceitando integracao, a plataforma vira app sem oferta.

### Fase 1 - MVP de dados e nota fiscal (2 a 5 meses)

- App de validade e inventario para laticinios e pao de forma.
- App consumidor com leitura de NFC-e.
- Pipeline de ingestao, deduplicacao e validacao.
- Cadastro EAN/GTIN multi-tenant.
- Wallet simples de pontos internos.
- Dashboard basico para lojas.
- Consentimento e governanca minima LGPD.

Nao incluir:

- token negociavel;
- conta financeira;
- delivery proprio;
- estoque complexo;
- full marketplace nacional.

### Fase 2 - Venda e integracao lojista (5 a 9 meses)

- Integracao de estoque/preco com lojas piloto.
- Pedido dentro da plataforma.
- Fluxo fiscal por vendedor.
- Separacao e entrega gerenciada pela loja.
- Busca georreferenciada.
- Cupons e campanhas patrocinadas.

### Fase 3 - Trade spend e analytics pago (9 a 15 meses)

- Motor de campanhas por categoria, busca, carrinho e localizacao.
- Relatorio de ROI por campanha.
- Analytics agregado por regiao/categoria.
- TENANT_DEFAULT curado e vendido como melhoria cadastral.
- Revisao fiscal escalavel.

### Fase 4 - Escala regional (15 a 24 meses)

- 100 lojas estrategicas.
- Modelo de cotas com vesting por performance.
- Parcerias com distribuidores.
- Delivery hibrido.
- Parceiro financeiro para conta/carteira, se a tracao justificar.

## 16. Arquitetura tecnica alvo

### Principios

- Multi-tenant desde o inicio.
- Event-driven para notas, pedidos, estoque, pagamentos e campanhas.
- Separacao forte entre dado identificavel e dado analitico.
- Auditoria nativa.
- Catalogo de produto versionado.
- Georreferenciamento como atributo central, nao feature secundaria.

### Componentes

- Identity e consentimento.
- Cadastro de pessoas e CNPJ.
- Catalogo global e catalogo por tenant.
- Ingestao NFC-e/OCR/QR Code.
- Normalizacao de itens.
- Wallet de pontos.
- Pedidos e checkout.
- Estoque e precos.
- Fiscal.
- Delivery.
- Campanhas/trade spend.
- Data warehouse.
- API externa B2B.
- Antifraude.
- Observabilidade e auditoria.

### Stack logica

- Banco transacional para operacao.
- DW/lakehouse para analytics.
- Busca textual e geoespacial.
- Fila/event bus.
- Feature store futura para recomendacao.
- Camada de BI para clientes.

## 17. O que esta fraco

### 1. A economia do scan ainda precisa ser provada

O usuario precisa de incentivo alto o bastante para escanear nota sempre. A fonte do incentivo esta definida no fundo por curva, mas a empresa ainda precisa provar que essa distribuicao aumenta retencao, compra e recorrencia.

Sem campanha patrocinada, SaaS, comissao propria ou venda incremental, o token vira redistribuicao sem retorno economico claro.

### 2. "Vender dados" e juridicamente perigoso

O plano precisa manter a linguagem em "inteligencia de mercado e precificacao". A empresa nao deve vender dado pessoal bruto, base de dados ou informacao reidentificavel. Se fizer isso, o risco LGPD pode matar a empresa.

### 3. Confundir fundo com receita distorce a decisao

O fundo de incentivo nao deve ser lido como receita da plataforma. Ele financia token de leitura, cupom de desconto e reserva operacional do proprio mecanismo. A empresa precisa de receita propria fora desse fundo.

### 4. Carteira, token e conta podem virar outra empresa

Produto financeiro regulado consome foco. Para MVP, pontos internos e saldo promocional sao suficientes.

### 5. Delivery proprio cedo demais e armadilha

Entrega tem SLA, roubo, cancelamento, perecivel, ruptura, substituicao e atendimento. Se entrar nisso cedo, o time vira operadora logistica antes de provar o motor de dados.

### 6. Integracao com loja e mais dificil do que parece

Cada loja pode ter ERP, PDV, cadastro, tributacao, estoque e processo diferente. A implantacao pode virar consultoria cara e pouco escalavel.

### 7. Cadastro de produto e fiscal e um buraco sem fundo

EAN, NCM, CEST, descricao, embalagem, unidade, tributacao por UF e substituicao tributaria exigem governanca pesada. Isso pode virar vantagem competitiva, mas tambem pode consumir o produto inteiro.

### 8. Industria so paga se houver prova de sell-out incremental

Nao basta mostrar banner na busca. Industria quer saber se vendeu mais por causa da campanha. Sem medicao de incrementalidade, o budget nao escala.

### 9. O consumidor pode nao querer mais um app

Escanear nota, criar conta, entender token e comprar por outro app e friccao alta. Precisa de recompensa imediata ou utilidade clara.

### 10. Ceder equity para lojas pode sair caro

Se lojas receberem cotas sem metas duras, a empresa perde cap table e nao ganha distribuicao proporcional.

### 11. O que nao fazer

- Nao virar um app genérico de cashback sem tese de varejo.
- Nao prometer dado bruto, base reidentificavel ou monetizacao fora de governanca LGPD.
- Nao incluir delivery proprio, carteira robusta e token negociavel no MVP.
- Nao usar fundo de incentivo como se fosse receita operacional.
- Nao dar equity amplo a loja sem meta, vesting e clawback.
- Nao tentar escalar 100 lojas antes de provar 3 integracoes boas.
- Nao depender de UX bonita para resolver friccao operacional.
- Nao competir com iFood, Mercado Livre ou redes grandes no front de marketplace horizontal.
- Nao lançar campanhas sem medicao de incrementalidade.
- Nao tratar consultado, anunciado ou declarado como verdade comercial final.

## 18. O que voce provavelmente nao esta vendo

### Antifraude sera central

Fraudes provaveis:

- mesma nota escaneada por varios usuarios;
- nota falsa;
- QR Code manipulado;
- usuario escaneando nota de terceiros;
- funcionario de loja coletando notas;
- bots;
- conluio entre loja e consumidor;
- campanha sendo explorada sem compra incremental.

Sem antifraude, o pool de tokens vira vazamento de caixa.

### Ruptura de estoque destrói conversao

Supermercado digital tem problema de item indisponivel, substituicao e preco divergente. Isso gera cancelamento, suporte e perda de confianca.

### O dado da NFC-e pode nao trazer tudo que voce espera

Dependendo da UF, layout, consulta, QR Code, disponibilidade e forma de captura, pode faltar EAN, descricoes podem vir ruins, itens podem ser abreviados e paginas podem bloquear automacao. Precisa validar tecnicamente em campo por estado.

Quando necessario para a estrategia de precificacao, a plataforma pode complementar a base com busca de precos em consultas publicas de SEFAZ estaduais, desde que a fonte permita uso tecnico e juridico. Exemplos a avaliar: SEFAZ do Parana, Bahia, Ceara e outros estados com servicos publicos semelhantes. Essa fonte deve ser tratada como complementar, nao como substituta da nota consentida, da integracao lojista ou do historico transacional proprio.

Algumas lojas cobrem a melhor oferta do mercado. A plataforma deve explorar isso comercialmente, mas com disciplina: preco consultado, anunciado ou declarado pela loja nao deve ser tratado como verdade final. A verdade comercial deve ser a venda efetiva confirmada, com item, preco, loja, data e contexto. Isso protege a confianca do consumidor e melhora a inteligencia de precificacao.

### Dados alimentares podem inferir saude

Compra recorrente de produtos sem acucar, suplementos, alcool, fraldas, medicamentos OTC ou alimentos especificos pode revelar informacao sensivel por inferencia. Isso exige cuidado maior que um simples app de cupom.

### Concorrente pode copiar o app, mas nao copia a coalizao rapido

O diferencial precisa ser contrato, dados, integracao, relacionamento regional e campanha paga. Interface isolada e copiavel.

### A governanca do TENANT_DEFAULT vira produto e risco

Quem valida? Quem responde por erro fiscal? Como versiona? Como prova origem? Como corrige retroativo? Isso precisa de processo desde cedo.

### O vendedor pode resistir a abrir dados

Lojista sabe que dado de venda e estrategico. Ele pode aceitar vender no app, mas resistir a expor estoque, preco e sell-out. A proposta precisa mostrar retorno claro.

### Industria pode preferir pagar a rede, nao a plataforma

Se a plataforma nao tiver volume suficiente, a industria vai continuar comprando trade direto com varejista grande. A entrada deve ser regional e focada em categorias onde verba de trade seja acessivel.

### O modelo de busca paga pode gerar conflito de confianca

Se o consumidor busca um produto e recebe propaganda demais, perde confianca. Patrocinio precisa ser transparente, util e economicamente vantajoso.

### Split financeiro e chargeback complicam conciliacao

Venda marketplace exige conciliacao entre consumidor, loja, entregador, plataforma, campanha subsidiada, cupom e impostos. Isso precisa nascer bem modelado.

## 19. Indicadores de sucesso

### Consumidor

- notas escaneadas por usuario por mes;
- custo por nota valida;
- usuarios ativos mensais;
- taxa de recompra;
- conversao de consulta para compra;
- resgate de pontos;
- fraude por 1.000 notas.

### Lojista

- GMV por loja;
- percentual do faturamento convertido para plataforma;
- ruptura;
- cancelamento;
- tempo de separacao;
- margem por pedido;
- aderencia de estoque/preco.

### Industria/distribuidor

- investimento por campanha;
- venda incremental;
- CAC por produto vendido;
- ROI por categoria;
- recorrencia de campanha.

### Convergencia de trade spend

Esses indicadores medem se o dinheiro de industria, distribuidor e loja esta convergindo para a plataforma como canal recorrente de ativacao comercial:

- verba de trade spend capturada por mes;
- numero de anunciantes ativos por mes;
- recorrencia de anunciantes apos a primeira campanha;
- percentual de campanhas renovadas;
- venda incremental atribuida por campanha;
- ROI por campanha, categoria, loja e regiao;
- custo de incentivo por venda incremental;
- margem incremental liquida apos cupom, cashback e subsidio;
- taxa de conversao de exposicao para carrinho;
- taxa de conversao de carrinho para compra;
- uplift de sell-out versus periodo-base ou grupo de controle;
- tempo medio entre ativacao da campanha e venda confirmada;
- participacao de campanhas no GMV da plataforma;
- verba media por anunciante;
- numero de categorias com campanhas recorrentes;
- percentual de campanhas com medicao confiavel de incrementalidade;
- recompra do consumidor apos campanha;
- diferenca entre preco anunciado, preco confirmado e preco final vendido.

### Plataforma

- receita por GMV;
- receita por loja;
- receita por consumidor ativo;
- margem de contribuicao;
- custo de suporte por pedido;
- custo de processamento por nota;
- churn de lojas;
- qualidade do cadastro EAN.

### Indicadores principais do projeto

Esses sao os indicadores de comando. Se eles nao evoluirem, o projeto nao deve escalar:

- lojas integradas ativas;
- GMV transacionado dentro da plataforma;
- percentual do faturamento da loja convertido para a plataforma;
- receita recorrente mensal da plataforma, sem contar fundo de incentivo;
- margem de contribuicao por pedido;
- custo total por pedido processado;
- notas fiscais validas por mes;
- custo por nota fiscal valida;
- percentual de notas com EAN utilizavel;
- usuarios ativos mensais;
- taxa de recompra em 30, 60 e 90 dias;
- uso de cupons emitidos pelo fundo;
- saldo/token convertido em compra;
- fraude por 1.000 notas;
- ruptura por loja e categoria;
- cancelamento por divergencia de preco ou estoque;
- tempo de implantacao por loja;
- aderencia de estoque/preco integrado;
- receita de trade spend por loja ativa;
- venda incremental paga por industria, distribuidor ou lojista.

## 20. Plano de validacao com decisoes objetivas

### Hipoteses a provar

- Consumidor escaneia nota por recompensa pequena.
- Loja aceita integrar estoque/preco.
- Industria paga por campanha com pouco volume inicial.
- Dados de nota sao tecnicamente capturaveis com qualidade suficiente.
- LGPD permite monetizacao agregada com consentimento e governanca.
- O fundo de incentivo por curva aumenta leitura de notas, conversao em compra e recorrencia sem destruir a margem.

### Experimentos

1. Teste de nota fiscal

Coletar 5.000 notas reais consentidas em 3 UFs. Medir taxa de leitura, EAN disponivel, qualidade de descricao, duplicidade e custo por nota.

2. Teste de incentivo

Testar a regra do fundo de incentivo por curva. Medir leitura de notas, uso de cupons, recompra, fraude, margem por pedido e impacto na recorrencia.

3. Teste lojista

Integrar 3 lojas antes de vender para 10. Medir tempo real de implantacao, qualidade do cadastro e ruptura.

4. Teste trade spend

Fechar 3 campanhas pagas pequenas. Medir clique, conversao, venda incremental e recompra do anunciante.

5. Teste fiscal

Executar 100 pedidos reais com emissao fiscal e conciliacao completa.

## 21. Decisao estrategica recomendada

Nao construir tudo ao mesmo tempo.

Sequencia correta:

1. Dados de nota fiscal + consentimento + catalogo EAN.
2. Lojas piloto + integracao simples.
3. Campanhas patrocinadas e cupons.
4. Venda dentro da plataforma.
5. Analytics pago.
6. Carteira com pontos internos.
7. Delivery hibrido.
8. Conta financeira ou token regulado somente depois de tracao.

O melhor wedge e "inteligencia e ativacao comercial para varejo alimentar regional", nao "super app de supermercado".

## 22. Conclusao executiva

A oportunidade existe, mas o plano esta grande demais para um MVP.

O que tem potencial:

- base de notas fiscais consentidas;
- catalogo EAN multi-tenant;
- TENANT_DEFAULT curado;
- revisao fiscal/cadastral;
- trade spend contextual;
- coalizao regional de lojas;
- dados agregados de sell-out.

O que precisa ser cortado ou adiado:

- token negociavel;
- conta estilo Caju;
- delivery proprio;
- marketplace nacional;
- promessa de venda de dados ou base bruta;
- equity amplo para lojas sem performance.

O plano fica forte se a empresa provar uma coisa: que consegue transformar dados de compra e estoque local em venda incremental paga por industria, distribuidor e lojista.

Se nao provar isso, vira um app caro de cupom e delivery competindo contra players maiores.

## 23. Cenarios de valuation em 24 meses

O valuation deve ser calculado principalmente sobre receita recorrente real, margem, retencao de lojas, recorrencia de campanhas e prova de venda incremental. GMV bruto nao deve ser usado como base principal de valuation, porque parte relevante do fluxo pode ser fundo de incentivo, cupom, repasse ao lojista ou subsidio comercial.

Premissa de referencia para 24 meses:

- 100 lojas ativas;
- GMV relevante dentro da plataforma;
- fundo de incentivo separado da receita da plataforma;
- receita real vinda de SaaS, campanhas, servicos, comissao propria e inteligencia de mercado e precificacao;
- trade spend recorrente e mensuravel;
- venda incremental comprovada.

Referencias de multiplos usadas como benchmark:

- SaaS Valuation Multiple, Q1 2026: mediana privada aproximada de 4,5x ARR e faixas maiores para empresas com crescimento, NRR e eficiencia superiores. Referencia: https://saasvaluationmultiple.com/
- iMerge Advisors, Q1 2026 Private SaaS Valuation Report: private SaaS estabilizado em aproximadamente 4,0x a 5,5x ARR, com premio para Rule of 40 e Net Revenue Retention. Referencia: https://imergeadvisors.com/saas-valuation-multiples-q1-2026/
- Flippa, Marketplaces Valuation Multiples for 2026: marketplaces digitais tendem a usar multiplos de receita na faixa de 2,0x a 4,0x, dependendo de efeito de rede, crescimento de GMV, margem e independencia operacional. Referencia: https://flippa.com/blog/marketplaces-valuation-multiples-what-do-you-need-to-evaluate/

### Cenario conservador

- Receita mensal: R$ 400k a R$ 700k.
- ARR: R$ 4,8M a R$ 8,4M.
- Multiplo estimado: 3x a 5x ARR.
- Valuation estimado: R$ 14M a R$ 42M.

Leitura: negocio validado, mas ainda com risco operacional alto, dependencia comercial relevante e prova limitada de escala.

### Cenario base forte

- Receita mensal: R$ 800k a R$ 1,5M.
- ARR: R$ 9,6M a R$ 18M.
- Multiplo estimado: 4x a 7x ARR.
- Valuation estimado: R$ 38M a R$ 126M.

Leitura: negocio com receita recorrente, campanhas pagas, retencao de lojas e sinais claros de venda incremental. Essa e a faixa mais defensavel se as metas forem batidas com consistencia.

### Cenario excelente

- Receita mensal: R$ 2M a R$ 3M.
- ARR: R$ 24M a R$ 36M.
- Multiplo estimado: 6x a 10x ARR.
- Valuation estimado: R$ 144M a R$ 360M.

Leitura: exige trade spend recorrente forte, baixa dependencia operacional, dados proprios dificeis de copiar, margem clara, crescimento mensal consistente e campanhas renovando sem venda consultiva pesada.

### Numero honesto de referencia

Se a plataforma bater as metas de 24 meses com receita recorrente, campanhas pagas e prova de incrementalidade, uma faixa defensavel fica entre R$ 60M e R$ 150M.

Acima disso, o valuation so se sustenta se a plataforma provar que converte dinheiro de trade spend em venda incremental mensuravel, com margem positiva e retencao real de lojas, industrias e consumidores.

## 24. Perguntas que precisam de resposta

- Quem paga primeiro: loja, industria ou consumidor?
- Qual e o wedge unico do MVP?
- O consumidor ganha o que em 30 segundos, nao em 30 dias?
- Qual dado voce consegue capturar com qualidade suficiente em 3 UFs?
- Qual taxa minima de leitura de nota torna o modelo viavel?
- Qual porcentagem de lojas realmente entrega estoque/preco integrado?
- Qual prova de incrementalidade faz a industria repetir campanha?
- O que entra no produto e o que fica explicitamente fora?
- Qual o custo de aquisicao de loja e de consumidor?
- Qual e o CAC payback esperado por segmento?
- Que parte do valor e SaaS, que parte e servico e que parte e midia?
- O que acontece se o fundo de incentivo cair pela metade?
- Como voce impede fraude de notas, bots e conluio?
- Como voce prova que nao esta vendendo dado pessoal bruto?
- Se so 20% das lojas integrarem bem, o negocio ainda existe?
- O que mata o projeto antes de escalar?
- Qual e o criterio objetivo para dizer "isso nao funciona, parar aqui"?
- Por que uma rede ou ERP nao copiaria isso em 12 meses?
- Qual e a primeira categoria que da mais valor com menos complexidade?
- O que voce esta vendendo de verdade: acesso, dados, ativacao ou transacao?

## 25. Matriz de decisao

| Manter | Cortar | Testar antes de escalar |
|---|---|---|
| Captura de nota fiscal consentida | Marketplace nacional | Taxa real de leitura de notas por UF |
| Catalogo EAN multi-tenant | Token negociavel | Qualidade do EAN por tipo de nota |
| TENANT_DEFAULT curado | Delivery proprio no inicio | Tempo de integracao por loja |
| Inteligencia de mercado e precificacao | Conta financeira estilo banco | CAC por loja, consumidor e industria |
| Campanhas patrocinadas / trade spend | Equity amplo para lojas sem performance | Incrementalidade real das campanhas |
| SaaS B2B para lojas | "App de supermercado" como tese central | Ticket que a industria paga para repetir campanha |
| Integracao fiscal minima | Complexidade fiscal total no MVP | Ruptura, cancelamento e divergencia de preco |
| Antifraude desde o dia 1 | Prometer dado bruto ou reidentificavel | Fraude por 1.000 notas |
| Venda incremental mensuravel | Depender do fundo de incentivo como receita | Recorrencia de uso do consumidor |
| Coalizao regional de lojas | Escalar 100 lojas antes de provar 3 | Tempo e custo de implantacao por loja |

## 26. Plano de elevacao, mitigacao e potenciais nao explorados

### Tese refinada

A empresa deve ser posicionada como uma infraestrutura regional de retail media, dados transacionais e ativacao comercial para o varejo alimentar independente.

O produto de entrada nao deve ser "app de supermercado" nem "cashback por nota". O wedge inicial deve ser:

- capturar nota fiscal consentida;
- normalizar item, EAN, loja, categoria, preco e data;
- integrar 3 lojas piloto com estoque/preco simples;
- ativar campanhas pagas por industria, distribuidor ou lojista;
- provar venda incremental por categoria, loja e regiao.

Se essa cadeia nao for provada, marketplace, wallet, delivery e analytics avancado devem continuar fora do escopo.

### ICP inicial recomendado

O primeiro cliente ideal deve ser loja ou rede regional com:

- 1 a 5 unidades;
- faturamento mensal relevante, mas baixa maturidade digital;
- ERP identificavel e acesso operacional ao cadastro de produtos;
- abertura para compartilhar estoque/preco com governanca;
- interesse em campanha local, cupom e sell-out incremental;
- decisor acessivel para piloto de 90 a 180 dias.

Lojas muito pequenas tendem a nao pagar o suficiente. Grandes redes tendem a ter area interna, fornecedor estabelecido e poder de negociacao maior. O melhor ponto de entrada e o varejo regional bom o bastante para ter dado, mas sem capacidade de construir a propria plataforma.

### Categoria inicial recomendada

O MVP deve escolher uma ou duas categorias antes de tentar cobrir o supermercado inteiro.

Categorias candidatas:

- bebidas nao alcoolicas;
- snacks, chocolates e biscoitos;
- higiene e beleza;
- limpeza domestica;
- mercearia seca;
- lacteos de alta recorrencia, se a operacao suportar ruptura e validade.

O criterio de escolha deve ser: recorrencia, verba de trade spend, simplicidade operacional, baixo risco de perecivel, EAN consistente e margem para incentivo.

### Gates de decisao

| Gate | Prazo | Criterio minimo | Decisao se falhar |
|---|---:|---|---|
| Juridico/LGPD | 30 a 60 dias | parecer permitindo uso consentido e inteligencia agregada | nao monetizar dados; reposicionar como software operacional |
| Nota fiscal | 60 a 90 dias | 5.000 notas reais, 3 UFs, taxa aceitavel de leitura e EAN utilizavel | limitar UFs/categorias ou abandonar scan como wedge principal |
| Lojista | 90 a 120 dias | 3 lojas integradas com estoque/preco ou carga operacional confiavel | nao vender para 10 lojas ainda |
| Trade spend | 90 a 180 dias | 3 campanhas pagas e pelo menos 1 renovacao | nao construir motor amplo de campanhas |
| Incentivo | 90 a 180 dias | recompensa aumenta scan, recompra ou conversao sem destruir margem | reduzir fundo ou mudar incentivo para cupom patrocinado |
| Fiscal/transacional | 120 a 180 dias | 100 pedidos com emissao fiscal e conciliacao completa | manter venda fora do MVP e focar dados/analytics |

### Registro de riscos e mitigacao

| Risco | Como aparece | Mitigacao |
|---|---|---|
| Consumidor nao escaneia nota | baixa recorrencia e custo alto por nota valida | recompensa imediata, cupom simples, limite antifraude e teste A/B por categoria |
| Dado da NFC-e ruim | falta EAN, descricao ruim, bloqueio por UF ou leitura inconsistente | validar por UF, manter fonte complementar SEFAZ quando permitido e priorizar integracao lojista |
| Lojista nao abre dados | resistencia a estoque, preco e sell-out | contrato claro, relatorio de retorno, campanha subsidiada e opcao de integracao gradual |
| Integracao vira consultoria | cada loja exige trabalho manual | padronizar conectores, aceitar CSV no piloto e medir tempo de implantacao por loja |
| Industria nao renova campanha | campanha gera clique, mas nao prova venda incremental | desenho com periodo-base, grupo de controle, cupom rastreavel e venda confirmada |
| LGPD limita monetizacao | risco de dado pessoal ou inferencia sensivel | consentimento granular, pseudonimizacao, descarte de CPF/nome e venda de insight agregado |
| Fraude consome fundo | nota duplicada, nota de terceiros, bot, funcionario coletando nota | chave unica da nota, device fingerprint, limite por usuario, score de risco e auditoria |
| Delivery distrai o time | SLA, ruptura, cancelamento e suporte crescem cedo demais | loja ou parceiro entrega no piloto; delivery proprio so depois de tracao |
| Equity para loja dilui demais | lojas recebem cotas sem entregar valor | vesting, metas, clawback e conversao apenas apos 12 meses de performance |
| Fundo vira falsa receita | incentivo mascara margem real | separar contabilidade do fundo, receita da plataforma e repasses desde o dia 1 |

### Potenciais ainda pouco explorados

1. Retail media regional

A plataforma pode virar uma rede de midia mensuravel para supermercados independentes. A proposta nao e apenas exibir anuncio, mas provar venda por loja, regiao, categoria e periodo.

2. Produto para distribuidores

Distribuidores podem financiar campanhas para acelerar giro, reduzir estoque parado, ativar marcas menores e ganhar leitura de demanda regional. Esse cliente pode pagar antes da industria grande.

3. Cadastro e fiscal como produto de entrada

EAN, descricao, NCM, CEST, unidade, embalagem e tributacao podem virar uma oferta B2B independente. A loja entra por melhoria cadastral e a plataforma cria base para campanha, busca e venda.

4. Benchmark regional de preco

A empresa pode vender indice agregado de competitividade por categoria, bairro, loja e periodo, sem expor dado pessoal ou dado bruto de consumidor.

5. Inteligencia de ruptura

Ruptura, substituicao, cancelamento e divergencia entre preco anunciado e preco vendido podem virar produto para loja, distribuidor e industria.

6. Campanhas cooperadas

Industria, distribuidor e loja podem dividir o incentivo de uma mesma campanha. Isso reduz dependencia de um pagador unico e melhora margem da plataforma.

7. Portal de marcas regionais

Marcas locais podem comprar ativacao em lojas especificas, com verba menor e ciclo comercial mais curto que grandes industrias nacionais.

8. API de catalogo

Se o TENANT_DEFAULT for bem governado, a plataforma pode oferecer enriquecimento de cadastro para ERPs, lojas, distribuidores e redes regionais.

9. Score operacional de loja

Aderencia de estoque/preco, ruptura, cancelamento, tempo de separacao, recompra e divergencia fiscal podem compor um score interno para priorizar campanhas e trafego.

10. Coalizao regional defensavel

O ativo defensavel nao e a interface. E a combinacao de contratos regionais, dados consentidos, integracao, catalogo curado, campanha paga e medicao de sell-out incremental.

### O que deve mudar no plano de execucao

- O MVP deve provar venda incremental antes de ampliar marketplace.
- A campanha paga deve ser vendida antes do motor completo de trade spend.
- O consumidor deve receber valor em ate 30 segundos depois do scan, mesmo que o beneficio financeiro maior venha depois.
- O dashboard lojista deve mostrar retorno pratico: venda, ruptura, preco, campanha e cadastro.
- O pitch para industria deve ser ROI e sell-out incremental, nao base de dados.
- O pitch para loja deve ser venda adicional, melhoria cadastral e canal regional, nao tecnologia.
- O pitch para distribuidor deve ser giro, campanha local e leitura de demanda.
- A contabilidade deve separar receita, incentivo, repasse, subsidio, cashback e reserva operacional.
- A arquitetura deve nascer com auditoria, consentimento, antifraude e separacao entre dado identificavel e dado analitico.
- A expansao para 100 lojas so deve ocorrer depois de 3 integracoes boas, 3 campanhas pagas e pelo menos 1 caso de renovacao.

### Proximos 30 dias

- escolher 1 UF principal e 2 UFs secundarias para teste de nota fiscal;
- selecionar 20 lojas-alvo e classificar por ERP, faturamento, categoria forte e abertura comercial;
- escolher 2 categorias iniciais para campanha;
- desenhar contrato-piloto com loja, industria/distribuidor e termos de consentimento;
- definir regra antifraude minima para nota, usuario, dispositivo, loja e campanha;
- montar planilha de unit economics separando receita real, fundo de incentivo e repasses;
- vender 3 campanhas pequenas antes de construir funcionalidades avancadas;
- definir criterios objetivos de continuidade, pausa ou encerramento do projeto.
