# Soroban Gatekeeper: Protokol Monetisasi Konten Desentralisasi

**Soroban Gatekeeper** adalah *smart contract* canggih yang dibangun di atas platform Soroban (Stellar Network). Proyek ini dirancang sebagai solusi *paywall* terdesentralisasi yang memungkinkan kreator konten untuk mengunci informasi sensitif (seperti kunci lisensi, link unduhan, atau pesan rahasia) dan hanya memberikannya kepada pengguna yang telah melakukan pembayaran.

## 🌟 Mengapa Aplikasi Ini Unik?
Berbeda dengan aplikasi catatan (notes) sederhana, Soroban Gatekeeper mengimplementasikan logika **Access Control List (ACL)** dan **Financial Tracking** secara *on-chain*:
- **Bukan Sekadar Penyimpanan**: Aplikasi ini mengatur hak akses dinamis.
- **Ekosistem Ekonomi**: Terdapat sistem pencatatan saldo internal untuk kreator.
- **Keamanan Ketat**: Menggunakan sistem otentikasi `require_auth` dari Soroban untuk memastikan data hanya terbuka bagi pembeli yang sah.

## 🛠️ Fitur Utama
1. **Content Vaulting**: Kreator dapat mengunggah konten dengan judul, isi rahasia, dan harga yang ditentukan sendiri.
2. **Permissioned Access**: Sistem secara otomatis memverifikasi apakah sebuah alamat dompet memiliki hak akses sebelum menampilkan data rahasia.
3. **Simulated Payment & Revenue**: Melacak transaksi pembelian dan mengakumulasikan saldo pendapatan bagi setiap kreator konten.
4. **Data Integrity**: Memanfaatkan struktur data `Map` dan `Vec` yang efisien untuk mengelola ribuan konten dan data pembeli tanpa tumpang tindih.

## 🏗️ Arsitektur Smart Contract

### Struktur Data
- `Content`: Menyimpan informasi tentang kreator, isi rahasia (*payload*), harga, dan judul.
- `ACCESS_LIST`: Map yang menghubungkan alamat `Address` pembeli dengan daftar ID konten yang telah mereka beli.
- `BALANCE`: Map yang mencatat total pendapatan yang dihasilkan oleh setiap alamat kreator.

### Fungsi Utama
- `post_content`: Digunakan oleh kreator untuk mendaftarkan konten baru ke dalam blockchain.
- `purchase_access`: Fungsi bagi pengguna untuk membeli akses. Fungsi ini akan memverifikasi otentikasi dan memperbarui hak akses serta saldo kreator.
- `unlock_content`: Fungsi krusial yang melakukan pengecekan hak akses. Jika pengguna belum membeli, kontrak akan memicu `panic!` (error) dan menolak menampilkan data.

## 📄 Informasi Deploy (Testnet)
- **Contract ID**: CAFWQWYZXNOWXABHSSNXLKNN6IGAWYC6IXFSY7CW5IWIJIH2UVJGFDNT
- **Network**: Stellar Testnet
- **Explorer**: [Lihat di Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CAFWQWYZXNOWXABHSSNXLKNN6IGAWYC6IXFSY7CW5IWIJIH2UVJGFDNT)

## 📝 Hasil Uji Coba (Invoke)
Kontrak telah berhasil diuji coba dengan fungsi `post_content`:
- **Title**: Materi
- **Payload**: LinkRahasia2026
- **Price**: 5

**Screenshot Berhasil:**
![Screenshot Invoke](screenshoot-invoke.png)

## 🚀 Cara Menjalankan & Menguji

### 1. Build Kontrak
Pastikan toolchain Rust dan Soroban CLI sudah terpasang:
```bash
soroban contract build
