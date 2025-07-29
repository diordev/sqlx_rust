/*

components — bu:

Ilova ichidagi **ko‘p martalik ishlatiladigan yordamchi qismlar** (helper tools, wrappers, trait extensions) joylashadigan modul.

Asosiy vazifasi — kodni **DRY** (Don't Repeat Yourself) prinsipiga mos holda yozishga yordam berish.

Bu yerda odatda quyidagilar bo‘ladi:
- Custom error tipi (`AppError`, `ApiError`)
- Response wrapper (`ApiResponse<T>`)
- Trait’lar (`Paginate`, `Validate`, `IntoResponse`, `FromRequest`)
- Auth helpers (JWT decoder, claims extractor)
- Context/Extension struct'lar (masalan, `AppState`, `AuthContext`)
- Custom middleware yoki guard’lar

Masalan:
- `components/error.rs` → umumiy xatoliklar strukturasini
- `components/response.rs` → yagona API response formatini taqdim etadi
- `components/pagination.rs` → sahifalash uchun trait

🎯 Afzalliklari:
✔️ Kodni modular qiladi
✔️ Yagona joyda o‘zgartirishni osonlashtiradi
✔️ Test yozish uchun qulay strukturani yaratadi

*/

pub mod state;
