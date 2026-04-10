Aqui está o **PRD completo unificado em Markdown**, já com a seção 3.4 integrada e padronizada — pronto para uso em README, Notion ou documentação técnica:

---

# PRD: DirectBlink Protocol (MVP)

**Versão:** 1.0
**Status:** Em Desenvolvimento (Fase de MVP)
**Stack:** Next.js 15, Tailwind, shadcn/ui, Anchor (Rust)

---

## 1. Visão Geral (The "Why")

O **DirectBlink** é uma infraestrutura de *social commerce* construída sobre a rede Solana. Ele permite que vendedores transformem postagens em redes sociais em checkouts nativos (*Blinks*), eliminando o redirecionamento para sites externos e reduzindo a taxa de abandono de carrinho.

### Objetivos Principais

* **Fricção Zero:** Comprar sem sair do feed (X, Instagram, WhatsApp)
* **Confiança Programática:** Preços e comissões garantidos por Smart Contracts
* **Viralidade Financeira:** Sistema de afiliados com pagamentos instantâneos via CPI

---

## 2. Público-Alvo

* **Vendedores (Merchants):** Criadores de conteúdo e pequenas lojas que vendem via redes sociais
* **Afiliados (Influencers):** Promotores que desejam comissões imediatas sem depender de aprovação manual
* **Compradores:** Usuários Web3 que buscam conveniência e segurança no checkout

---

## 3. Requisitos Funcionais (MVP)

### 3.1. Dashboard do Vendedor (Admin)

* **Gestão de Produtos:** Criar, editar e listar produtos que residem em PDAs
* **Gerador de Blinks:** Interface para gerar e copiar o link da Action
* **Analytics:** Visualização de vendas, receita total e performance de afiliados

---

### 3.2. Motor On-chain (DBLK_Engine)

* **Product Registry:** Registro de *Price*, *Stock* e *Metadata* na blockchain
* **Atomic Checkout:** Processamento de pagamento com divisão automática de taxas (*Fee Splitting*)
* **Affiliate Protocol:** Vinculação de carteiras de afiliados a produtos com comissão fixa

---

### 3.3. Sistema de Pagamentos

* **Taxa de Protocolo:** Retenção automática de **0.75%** sobre cada venda
* **Suporte a SPL Tokens:** Inicialmente focado em **USDC** para estabilidade
* **Recibos Digitais:** Notificação de sucesso com link para o Solana Explorer *(roadmap para cNFTs)*

---

### 3.4. Permissionless Affiliate Onboarding (MVP Feature)

Para garantir que o **DirectBlink** escale de forma orgânica e agressiva, o sistema de afiliados adota um modelo de auto-serviço *on-chain*.

#### Descrição

Qualquer usuário com uma carteira Solana pode se tornar um promotor de qualquer produto cadastrado no protocolo, sem necessidade de autorização prévia do vendedor.

---

#### Mecanismo de Incentivo

O afiliado é responsável por arcar com o custo de **Rent (aluguel)** da sua própria PDA de afiliado (*AffiliateState*).

Isso garante que o protocolo não gere custos para o vendedor original enquanto a rede cresce.

---

#### Fluxo de Execução

1. O afiliado identifica um produto via **Dashboard** ou **Explorer**
2. O afiliado assina a instrução `onboard_affiliate`, inicializando sua PDA vinculada ao produto
3. A partir desse momento, qualquer transação via **Blink** que inclua a **Pubkey** desse afiliado no parâmetro da URL irá disparar o **Atomic Fee Split**

---

## 🛠️ Impacto na Engenharia (Requisito Técnico)

| Requisito            | Detalhe Técnico                                                                                                                         |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| Integridade de Dados | A PDA do afiliado deve ser derivada de `ProductState.key()` e `Affiliate.key()` para evitar duplicidade                                 |
| Proteção de Margem   | A comissão (`commission_bps`) no MVP é sugerida pelo protocolo ou limitada pelo vendedor na criação do produto                          |
| Segurança de Saque   | Não há botão de saque; o pagamento é enviado via **CPI (Cross-Program Invocation)** diretamente para a carteira do afiliado no checkout |

---

## 4. Arquitetura Técnica

### 4.1. On-chain (Solana / Anchor)

| Componente          | Função                                        |
| ------------------- | --------------------------------------------- |
| ProductState PDA    | Armazena a "Fonte da Verdade" do item         |
| AffiliateState PDA  | Relação Vendedor-Afiliado e ganhos acumulados |
| CPI (Token Program) | Executa transferências atômicas de fundos     |

---

### 4.2. Off-chain (Next.js / Edge Functions)

* **Actions API:** Implementação da especificação de Action da Solana para renderizar botões de compra
* **Metadata Storage:** Integração com IPFS/Arweave para imagens dos produtos

---

## 5. Requisitos Não-Funcionais

* **Performance:** API de Actions deve responder em **< 200ms** para renderização fluida no X/Twitter
* **Segurança:** Todas as alterações de estado exigem assinatura do *Signer (Vendedor)*
* **Escalabilidade:** Uso eficiente de PDAs para minimizar custo de *rent* na rede

---

## 6. Roadmap do MVP

### Semana 1: Core Build

* Finalização do **DBLK_Engine (Anchor)**
* Deploy na **Devnet**
* Implementação da **API de Actions básica**

---

### Semana 2: Merchant Experience

* Construção do Dashboard em **Next.js + shadcn**
* Integração com **Solana Wallet Adapter**

---

### Semana 3: Viral Loop & Launch

* Sistema de afiliados funcional
* Testes de carga e submissão (*Ideathon*)

---

## 7. Métricas de Sucesso (KPIs)

* **Time to Market:** Publicação funcional na Mainnet
* **Conversão:** Checkout em menos de **10 segundos**
* **Volume Transacionado (TVL):** Primeiros **$1.000 em USDC** processados

---

