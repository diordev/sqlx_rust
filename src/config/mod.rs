/*

config — bu:

Tashqi tizimlar bilan integratsiya qiluvchi **konfiguratsiya modullari** joylashadigan joy.

Bu modulda odatda quyidagilar bo‘ladi:
- `.env` fayldagi ma'lumotlarni o‘qish
- Database (PostgreSQL, MySQL, SQLite)
- Redis
- RabbitMQ
- Kafka
- Logger (tracing) konfiguratsiyasi
- Global konfiguratsiya struct (`AppConfig`, `Config`) shaklida

Vazifalari:
✔️ Har bir servisga mos `connection_string` yoki `connect()` funksiyasini taqdim etadi
✔️ Ulanish parametrlarini `Option<T>` bilan ishlatib, **moslashuvchan** qiladi
✔️ `.env` ni `.dotenvy` yoki `env` orqali **bir marta yuklaydi**

💡 Bu qatlamni alohida saqlash orqali siz konfiguratsiyani markazlashtirasiz va kodni testga qulaylashtirasiz.

Masalan:
- `config/database.rs` – DB connection
- `config/kafka.rs` – Kafka settings
- `config/mod.rs` – `Config::load()` yordamida barcha sozlamalarni to‘playdi

*/

pub mod database;
