# 🚀 Teste Técnico - PicPay

### Projeto simples de sistema de transferência via protocolo HTTP.

---

## 💡 Sobre o Projeto

Este projeto implementa uma API REST básica para simular transferências entre usuários.  
A proposta é criar uma estrutura funcional e organizada, utilizando Rust e ferramentas modernas da stack web assíncrona.  
O objetivo é atender aos requisitos do teste com clareza e manter o código limpo e extensível para futuras melhorias.

---

## 🛠️ Tecnologias Utilizadas

- **Rust**
- **Axum** – Framework web assíncrono
- **Serde** – Serialização e desserialização
- **SQLx** – Integração com PostgreSQL
- **PostgreSQL**

---

## 🔁 Como Fazer uma Transferência

Use no PowerShell:

```powershell
Invoke-RestMethod -Uri http://localhost:3000/transfer `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"payer_id":1,"payee_id":2,"value":100.50}'
```

## 🔁 Como ver os usuarios ?

Use no PowerShell:

```powershell
Invoke-RestMethod -Uri http://localhost:3000/user/1 -Method GET
```

Link do desafio:
https://github.com/PicPay/picpay-desafio-backend
