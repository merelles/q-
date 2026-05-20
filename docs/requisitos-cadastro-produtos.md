# Requisitos - Cadastro de produtos

Data base: 2026-05-16  
Status: rascunho controlado para modelagem do core de catalogo

## 1. Objetivo

Definir os requisitos do cadastro de produtos da plataforma, considerando a ingestao de bases externas de GTIN, leitura de nota fiscal, cadastro multi-tenant, curadoria especializada, integracoes de terceiros e cadastro de imagem de produto.

O cadastro nao deve ser tratado como uma tabela simples de produtos. Ele sera um ativo governado, normalizado, auditavel e preparado para suportar operacao, inteligencia comercial, promocao preventiva, fiscal, estoque, integracao com lojistas e integracoes B2B externas.

## 2. Premissas

- O GTIN/EAN identifica codigos comerciais, mas nao resolve sozinho a verdade do produto.
- A mesma descricao pode variar por fonte, loja, ERP, nota fiscal e idioma.
- Um produto pode nascer como proposta e somente depois virar produto validado.
- A base curada deve preservar evidencias de origem, decisao humana e rastreabilidade.
- Produtos locais por tenant podem existir antes de uma padronizacao global.
- O modelo precisa suportar integracao com fontes externas como Open4Goods, GS1, notas fiscais, ERP de lojista e cadastro manual.
- Integracao representa terceiro com acesso controlado ao cadastro, nao usuario interno.
- O cadastro de imagem e parte do cadastro de produto e deve seguir governanca de origem e auditoria.
- O escopo precisa permanecer aderente ao plano de negocio em `docs/plano-negocio-plataforma-varejo-dados.md`, com foco em infraestrutura de dados e transacao para varejo alimentar.

## 3. Estados do produto

### 3.1 Produto proposto

Produto ainda nao aprovado pelo time especialista.

Requisitos:

- Deve possuir origem identificavel.
- Deve guardar nome bruto e nome normalizado.
- Deve aceitar um ou mais GTINs candidatos.
- Deve registrar evidencias usadas na proposta.
- Deve ter status de workflow.
- Deve poder ser rejeitado sem apagar historico.
- Deve poder voltar para ajuste quando a curadoria pedir correcao.

### 3.2 Produto em revisao

Produto aguardando decisao humana ou regra automatizada supervisionada.

Requisitos:

- Deve bloquear promocao, analytics vendavel e uso como TENANT_DEFAULT ate aprovacao.
- Deve exibir conflitos de nome, marca, unidade, categoria, origem e GTIN.
- Deve registrar responsavel, data, decisao e justificativa.
- Deve permitir comparacao entre fontes.

### 3.3 Produto validado

Produto aprovado para compor a base curada.

Requisitos:

- Deve possuir nome normalizado e criterio de normalizacao.
- Deve possuir relacionamento rastreavel com GTINs ativos.
- Deve possuir escopo claro: global curado ou local do tenant.
- Deve possuir status ativo/inativo.
- Deve manter historico de alteracoes.
- Deve aceitar enriquecimento posterior sem perda de auditoria.

### 3.4 Produto rejeitado ou depreciado

Produto que nao pode ser usado como base validada.

Requisitos:

- Deve manter motivo da rejeicao ou depreciacao.
- Deve manter vinculo com proposta, fonte e evidencias.
- Nao deve ser apagado fisicamente em operacao normal.
- Deve poder ser usado para antifraude, deduplicacao e aprendizado de curadoria.

## 4. Requisitos funcionais

### RF-001 - Cadastro de GTIN

O sistema deve cadastrar GTINs com tipo, pais GS1 quando disponivel, fonte, prefixo, referencia do item, digito verificador e status ativo.

Critérios de aceite:

- GTIN deve aceitar 8 a 14 digitos numericos.
- GTIN deve ser unico na base.
- Fonte deve ser obrigatoria.
- Pais deve ser opcional, mas validado quando informado.

### RF-002 - Cadastro de fontes

O sistema deve manter fontes de dados com codigo unico, descricao e identificador interno.

Critérios de aceite:

- Cada registro importado deve apontar para uma fonte.
- Deve ser possivel distinguir fonte externa, nota fiscal, ERP, operador humano e curadoria.

### RF-003 - Cadastro de marcas

O sistema deve cadastrar marcas com nome unico e permitir associacao entre GTIN e marca.

Critérios de aceite:

- Marca deve ser normalizada antes de virar referencia curada.
- GTIN pode ter conflito de marca entre fontes.
- Conflitos devem ir para revisao.

### RF-004 - Produto por tenant

O sistema deve permitir produto local por tenant.

Critérios de aceite:

- Tenant deve ser obrigatorio para produto local.
- Nome deve ser unico por tenant enquanto ativo.
- Produto local nao deve sobrescrever produto global curado.

### RF-005 - Produto global curado

O sistema deve manter uma base global curada para padronizacao e analytics.

Critérios de aceite:

- Produto global deve exigir validacao.
- Alteracao em produto global deve gerar auditoria.
- Produto global deve preservar aliases e fontes de origem.

### RF-006 - Proposta de produto

O sistema deve permitir criar proposta de produto a partir de GTIN, nota fiscal, base externa, ERP ou cadastro manual.

Critérios de aceite:

- Proposta deve nascer em status controlado.
- Proposta deve manter evidencias.
- Proposta deve possuir trilha de workflow.

### RF-007 - Workflow de curadoria

O sistema deve controlar transicoes entre rascunho, proposto, em revisao, validado, rejeitado e depreciado.

Critérios de aceite:

- Transicoes invalidas devem ser bloqueadas.
- Decisoes devem ter ator, data e justificativa.
- Rejeicao e depreciacao nao devem apagar dados historicos.

### RF-008 - Normalizacao de nomes

O sistema deve manter nome bruto e nome normalizado.

Critérios de aceite:

- Nome bruto deve preservar a origem.
- Nome normalizado deve seguir regra documentada.
- Mudanca de regra de normalizacao deve ser versionada.

### RF-009 - Associacao produto-GTIN

O sistema deve associar GTINs a produtos.

Critérios de aceite:

- Um GTIN ativo nao deve apontar para mais de um produto ativo na mesma camada curada.
- Historico de associacao deve ser preservado.
- Mudanca de associacao deve exigir justificativa.

### RF-010 - Unidade de medida

O sistema deve manter unidades de medida normalizadas.

Critérios de aceite:

- Unidade deve ter codigo, simbolo, descricao e casas decimais.
- Unidade deve suportar produtos unitarios, peso, volume e quantidade fracionada.

### RF-011 - Evidencia de origem

O sistema deve guardar evidencia suficiente para explicar por que um produto foi criado, alterado ou validado.

Critérios de aceite:

- Evidencia deve apontar fonte.
- Evidencia pode guardar referencia externa, nome bruto, marca bruta e hash do payload.
- Evidencia nao deve expor dado pessoal desnecessario.

### RF-012 - Auditoria

O sistema deve registrar eventos relevantes de cadastro, proposta, validacao, rejeicao, depreciacao e associacao de GTIN.

Critérios de aceite:

- Evento deve ter agregado, identificador, acao, ator e justificativa quando aplicavel.
- Auditoria deve ser imutavel para operacao comum.
- Auditoria deve suportar rastreabilidade de decisao.

### RF-013 - Integracao de terceiros

O sistema deve permitir cadastro e governanca de integracoes terceiras com acesso ao catalogo.

Critérios de aceite:

- Integracao deve ser vinculada a tenant.
- Integracao deve ter nivel de acesso minimo entre leitura, escrita de catalogo e escrita de catalogo+midia.
- Integracao deve possuir status (ativo, suspenso, revogado).
- Operacoes realizadas por integracao devem gerar trilha de auditoria.

### RF-014 - Cadastro de imagem de produto

O sistema deve permitir associar imagens ao produto por fluxo interno ou integracao autorizada.

Critérios de aceite:

- Imagem deve possuir URL ou storage key, identificador e opcionalmente texto alternativo.
- Produto pode ter multiplas imagens, com apenas uma primaria por vez.
- Inclusao, troca de primaria e desativacao de imagem devem ser auditadas.
- Produto em estado proposto pode receber imagem, mas promocao exige produto validado.

### RF-015 - API de catalogo

O sistema deve expor uma API versionada para consulta e manutencao de cadastro de produto.

Critérios de aceite:

- API deve expor endpoints de produto, GTIN, imagem e integracao.
- API deve expor contrato OpenAPI/Swagger.
- API deve padronizar erros de validacao e nao encontrado.
- API deve permitir evolucao sem quebra de contrato via versionamento.

### RF-016 - Frontend de operacao de catalogo

O sistema deve ter frontend para operacao de cadastro de produto aderente ao UI kit e style guide institucional.

Critérios de aceite:

- Frontend deve consumir a API de catalogo.
- Frontend deve permitir cadastro minimo de produto, consulta e anexacao de imagem.
- Frontend deve respeitar tema, tokens e componentes do UI kit adotado.
- Frontend deve funcionar em desktop e mobile.

## 5. Requisitos nao funcionais

### RNF-001 - Qualidade de produto de software

O cadastro deve ser desenvolvido considerando ISO/IEC 25010:2023 como referencia de qualidade de produto de software.

Aplicacao pratica:

- Adequacao funcional: regras de GTIN, produto, proposta e curadoria devem ser testaveis.
- Confiabilidade: importacoes e workflows devem ser idempotentes sempre que possivel.
- Usabilidade: telas futuras de curadoria devem expor conflitos de forma clara.
- Eficiencia de desempenho: busca por GTIN, produto e tenant deve ser indexada.
- Manutenibilidade: dominio deve ficar separado de banco, API e importadores.
- Segurança: operacoes de curadoria devem exigir autorizacao e auditoria.
- Compatibilidade: core deve aceitar diferentes fontes e ERPs.
- Portabilidade: regras de dominio devem funcionar fora de um banco especifico.

### RNF-002 - Sistema de gestao da qualidade

O processo deve considerar ISO 9001:2015 como referencia para gestao da qualidade.

Aplicacao pratica:

- Requisitos devem ser versionados.
- Mudancas de regra devem ter responsavel e justificativa.
- Indicadores de qualidade cadastral devem ser acompanhados.
- Nao conformidades devem gerar acao corretiva.
- Decisoes de curadoria devem ser rastreaveis.

### RNF-003 - MPS-BR

O desenvolvimento deve considerar o Guia Geral MPS de Software:2023 da Softex como referencia de maturidade de processo.

Aplicacao pratica:

- Requisitos devem ter criterios de aceite.
- Mudancas devem passar por controle.
- Evidencias de teste, revisao e aceite devem ser preservadas.
- Riscos de dados, LGPD, fiscal e operacao devem ser registrados.
- Entregas devem ser planejadas, verificadas e medidas.

### RNF-004 - LGPD e minimizacao

O cadastro de produtos deve evitar persistir dado pessoal.

Aplicacao pratica:

- Dados de nota fiscal usados como evidencia devem ser minimizados.
- CPF, nome de consumidor e identificadores pessoais nao devem entrar na base de produto.
- Hash de payload pode ser usado para rastreabilidade quando o payload bruto for sensivel.

### RNF-005 - Integridade e consistencia

O sistema deve priorizar integridade referencial e consistencia historica.

Aplicacao pratica:

- Chaves estrangeiras devem proteger pais, fonte, tipo de GTIN, tenant, marca e produto.
- Exclusoes fisicas devem ser excecao.
- Status ativo/inativo deve preservar historico.
- Dados externos devem entrar primeiro como proposta ou staging, nao diretamente como verdade curada.

## 6. Requisitos de dados

### 6.1 Entidades minimas

- Country
- Source
- Tenant
- Brand
- GtinType
- Unit
- Gtin
- GtinBrand
- Product
- ProductGtin
- ProductImage
- ProductProposal
- ProductEvidence
- SpecialistReview
- ProductAuditEvent
- Integration

### 6.2 Campos adicionais recomendados

- status de revisao em produto e proposta;
- nome bruto e nome normalizado;
- hash de payload externo;
- origem da proposta;
- usuario ou servico responsavel pela decisao;
- justificativa de validacao, rejeicao ou depreciacao;
- timestamps de criacao, atualizacao, validacao e depreciacao;
- versionamento de regra de normalizacao;
- score de confianca por fonte.
- nivel de acesso e status de integracao.
- metadados de imagem (url ou storage key, primaria, origem).

## 7. Requisitos de governanca

- Toda promocao baseada em produto deve usar produto validado ou regra explicita de excecao.
- Produto proposto nao deve alimentar inteligencia vendavel sem sinalizacao de qualidade.
- TENANT_DEFAULT deve ser tratado como camada curada, nao como copia automatica de um tenant.
- Importacao massiva deve gerar relatorio de qualidade: registros aceitos, rejeitados, duplicados, conflitantes e pendentes.
- Curadoria deve ter amostragem, dupla revisao para casos criticos e trilha de auditoria.
- Alteracoes em GTIN-produto devem ser reversiveis por novo evento, nao por edicao silenciosa.
- Integracao externa deve operar com principio do menor privilegio.
- Mudancas por integracao devem ser segregadas de mudancas manuais para fins de compliance.

## 8. Indicadores de qualidade cadastral

- Percentual de GTINs com produto validado.
- Percentual de produtos com marca validada.
- Percentual de produtos com unidade validada.
- Taxa de conflito por fonte.
- Tempo medio de revisao de proposta.
- Taxa de rejeicao por motivo.
- Duplicidade detectada por tenant.
- Cobertura por categoria prioritaria.
- Produtos usados em promocao preventiva sem validacao completa.
- Incidentes de dado pessoal em payload de evidencia.
- Taxa de erro por integracao terceira no cadastro.
- Percentual de produtos com imagem primaria valida.

## 9. Riscos

- Tratar Open4Goods como verdade, quando deve ser fonte de proposta.
- Confundir GTIN com produto final.
- Validar produto sem evidencia suficiente.
- Permitir que cada tenant crie uma ontologia propria sem ponte para a base global.
- Escalar importacao antes de ter curadoria, auditoria e criterios de qualidade.
- Expor dado de nota fiscal alem do necessario.
- Vender analytics com base de baixa confianca.
- Conceder acesso excessivo para integracao terceira.
- Permitir imagem sem origem rastreavel no cadastro curado.

## 10. Decisoes de arquitetura

- `catalog/product-core` deve conter structs, enums e traits de dominio.
- Banco, migrations, importadores e APIs devem ficar fora do core.
- O core deve expressar estados e transicoes permitidas.
- Repositorios devem ser traits para permitir PostgreSQL, memoria e testes.
- Integracao com Open4Goods deve criar propostas e evidencias, nao produtos validados automaticamente.
- API pode viver em `catalog/catalog-service` (ou nome equivalente) com contrato Swagger para consumo por frontend e integracoes.
- Frontend de catalogo deve consumir essa API e usar o UI kit `zixel` como padrao visual.

## 11. Referencias

- ISO/IEC 25010:2023 - Product quality model: https://www.iso.org/standard/78176.html
- ISO 9001:2015 - Quality management systems requirements: https://www.iso.org/standard/62085.html
- Guia Geral MPS de Software:2023 - Softex: https://softex.br/download/guia-geral-mps-de-software2023/
