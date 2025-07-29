/*

db — bu:

Ma’lumotlar ombori (PostgreSQL, Redis, Kafka, RabbitMQ va h.k.) bilan ishlashga javobgar qatlam.

Vazifalari quyidagilardan iborat:

- `Database`, `Redis`, `Kafka`, `RabbitMQ` kabi xizmatlar bilan ulanishni boshqaradi
- Ulanish uchun kerakli `connect()` metodlarini taqdim etadi
- `sqlx` uchun `PgPool`, Redis uchun client va boshqalarni abstraksiyalab beradi
- `config` orqali olingan ma’lumotlardan foydalangan holda ulanish stringlarini generatsiya qiladi

Bu modul dasturga xizmatlar bilan **mustahkam, configurable** tarzda bog‘lanish imkonini beradi.

💡 Maslahat:
`models` faqat ma’lumot tuzilmasi bo‘lsa, `db` real ulanishlarni, ya’ni ishlovchi connection va pool’larni tashkil qiladi.

*/
pub mod pool;
