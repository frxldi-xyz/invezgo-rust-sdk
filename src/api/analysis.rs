//! Endpoints for analysis API

use crate::client::InvezgoClient;
use crate::models::analysis::*;

pub struct AnalysisApi {
    pub(crate) client: InvezgoClient,
}

impl AnalysisApi {

    /// Mendapatkan daftar lengkap semua perusahaan yang tercatat di Bursa Efek Indonesia (BEI).
    /// 
    ///     Fitur:
    ///     - Termasuk perusahaan IPO terbaru
    ///     - Tidak termasuk perusahaan yang sudah delisting
    ///     - Dilengkapi dengan logo perusahaan
    /// 
    ///     Penggunaan:
    ///     - Untuk mendapatkan kode saham yang valid
    ///     - Sebagai referensi untuk endpoint lainnya
    ///     - Untuk menampilkan daftar saham di website
    /// 
    ///     Update: Setiap ada perubahan data perusahaan atau IPO Baru
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a complete list of all companies listed on the Indonesia Stock Exchange (IDX).
    /// 
    ///     Features:
    ///     - Includes latest IPO companies
    ///     - Excludes delisted companies
    ///     - Complete with company logos
    /// 
    ///     Usage:
    ///     - To get valid stock codes
    ///     - As reference for other endpoints
    ///     - To display stock list on website
    /// 
    ///     Update: Every time there is a change in company data or new IPO
    pub async fn get_stock_list(&self) -> Result<Vec<GetStockListResponse>, crate::error::InvezgoError> {
        let path = "/analysis/list/stock".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar lengkap semua broker/sekuritas yang terdaftar sebagai anggota bursa di BEI.
    /// 
    ///     Fitur:
    ///     - Mengikuti perkembangan MKBD (Member of Broker Dealer)
    ///     - Kode broker 2 digit untuk identifikasi
    ///     - Nama lengkap perusahaan sekuritas
    /// 
    ///     Penggunaan:
    ///     - Untuk mendapatkan kode broker yang valid
    ///     - Sebagai referensi untuk analisis transaksi broker
    ///     - Untuk filter data berdasarkan broker tertentu
    /// 
    ///     Update: Setiap ada perubahan data broker atau ada broker baru
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a complete list of all brokers/securities registered as exchange members at IDX.
    /// 
    ///     Features:
    ///     - Follows MKBD (Member of Broker Dealer) updates
    ///     - 2-digit broker code for identification
    ///     - Full name of securities company
    /// 
    ///     Usage:
    ///     - To get valid broker codes
    ///     - As reference for broker transaction analysis
    ///     - To filter data based on specific broker
    /// 
    ///     Update: Every time there is a change in broker data or new broker
    pub async fn get_broker_list(&self) -> Result<Vec<GetBrokerListResponse>, crate::error::InvezgoError> {
        let path = "/analysis/list/broker".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar lengkap semua index/indeks yang tersedia di BEI.
    /// 
    ///     Kategori Index:
    ///     - **headline**: Index utama (COMPOSITE/IHSG, LQ45, IDX30, IDX80)
    ///     - **sector**: Index sektoral (IDXENERGY, IDXFINANCE, IDXBASIC, dll)
    ///     - **sharia**: Index syariah (JII, JII70, ISSI, dll)
    ///     - **esg**: Index ESG/sustainability (SRI-KEHATI, IDXESGL, dll)
    ///     - **factor**: Index berbasis faktor (IDXQ30, IDXV30, IDXG30, IDXHIDIV20)
    ///     - **thematic**: Index tematik (IDXBUMN20, I-GRADE, dll)
    ///     - **board**: Index papan pencatatan (MBX, DBX, ABX)
    ///     - **partnership**: Index kerjasama (KOMPAS100, BISNIS-27, dll)
    ///     - **smc**: Index Small-Mid Cap (IDXSMC-LIQ, IDXSMC-COM)
    /// 
    ///     Penggunaan:
    ///     - Untuk mendapatkan kode index yang valid
    ///     - Sebagai referensi untuk analisis pergerakan index
    ///     - Untuk filter data berdasarkan kategori index
    /// 
    ///     Update: Setiap ada perubahan data index
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a complete list of all indexes available at IDX.
    /// 
    ///     Index Categories:
    ///     - **headline**: Main indexes (COMPOSITE/IHSG, LQ45, IDX30, IDX80)
    ///     - **sector**: Sectoral indexes (IDXENERGY, IDXFINANCE, IDXBASIC, etc)
    ///     - **sharia**: Sharia indexes (JII, JII70, ISSI, etc)
    ///     - **esg**: ESG/sustainability indexes (SRI-KEHATI, IDXESGL, etc)
    ///     - **factor**: Factor-based indexes (IDXQ30, IDXV30, IDXG30, IDXHIDIV20)
    ///     - **thematic**: Thematic indexes (IDXBUMN20, I-GRADE, etc)
    ///     - **board**: Board indexes (MBX, DBX, ABX)
    ///     - **partnership**: Partnership indexes (KOMPAS100, BISNIS-27, etc)
    ///     - **smc**: Small-Mid Cap indexes (IDXSMC-LIQ, IDXSMC-COM)
    /// 
    ///     Usage:
    ///     - To get valid index codes
    ///     - As reference for index movement analysis
    ///     - To filter data based on index category
    /// 
    ///     Update: Every time there is a change in index data
    pub async fn get_index_list(&self) -> Result<Vec<GetIndexListResponse>, crate::error::InvezgoError> {
        let path = "/analysis/list/index".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan informasi lengkap dan detail tentang perusahaan berdasarkan kode saham.
    /// 
    ///     Informasi yang Disediakan:
    ///     - Data perusahaan (nama, alamat, website)
    ///     - Klasifikasi industri dan sektor bisnis
    ///     - Tanggal listing di bursa
    ///     - NPWP dan informasi legal
    /// 
    ///     Jenis Data: Cross Section (data snapshot terbaru)
    /// 
    ///     Update: Bulanan sesuai laporan masing-masing perusahaan
    /// 
    ///     Contoh Penggunaan:
    ///     - Untuk menampilkan profil perusahaan di website
    ///     - Sebagai referensi untuk analisis fundamental
    ///     - Untuk validasi data perusahaan
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get complete and detailed information about a company based on stock code.
    /// 
    ///     Information Provided:
    ///     - Company data (name, address, website)
    ///     - Industry classification and business sector
    ///     - Listing date on exchange
    ///     - Tax ID and legal information
    /// 
    ///     Data Type: Cross Section (latest snapshot data)
    /// 
    ///     Update: Monthly according to each company's report
    /// 
    ///     Usage Examples:
    ///     - To display company profile on website
    ///     - As reference for fundamental analysis
    ///     - To validate company data
    pub async fn information(&self, code: &str) -> Result<InformationResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/information/{}", code);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar saham dengan kenaikan dan penurunan harga terbesar pada hari tertentu, dilengkapi dengan data grafik 5 hari ke belakang.
    /// 
    ///     Fitur:
    ///     - Top Gainer: Saham dengan kenaikan perubahan harga tertinggi
    ///     - Top Loser: Saham dengan penurunan perubahan harga terbesar
    ///     - Graph: Data perubahan harga 5 hari ke belakang untuk setiap saham
    /// 
    ///     Jenis Data: Cross Section + Time Series (data harian + grafik)
    /// 
    ///     Update: End of Day (EOD) setiap hari setelah jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Catatan:
    ///     - Data tidak tersedia pada hari libur atau weekend
    ///     - Gunakan tanggal hari kerja untuk hasil terbaik
    ///     - Setiap saham dilengkapi dengan data grafik 5 hari
    /// 
    ///     Contoh Penggunaan:
    ///     - Dashboard Top Gainer/Loser dengan mini chart
    ///     - Analisa momentum harian dengan tren
    ///     - Identifikasi saham yang trending dengan visualisasi
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of stocks with the largest price increases and decreases on a specific day, complete with 5-day historical graph data.
    /// 
    ///     Features:
    ///     - Top Gainer: Stocks with the highest price change increase
    ///     - Top Loser: Stocks with the largest price change decrease
    ///     - Graph: 5-day historical price change data for each stock
    /// 
    ///     Data Type: Cross Section + Time Series (daily data + graph)
    /// 
    ///     Update: End of Day (EOD) every day after 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Notes:
    ///     - Data not available on holidays or weekends
    ///     - Use business days for best results
    ///     - Each stock includes 5-day graph data
    /// 
    ///     Usage Examples:
    ///     - Top Gainer/Loser Dashboard with mini charts
    ///     - Daily momentum analysis with trends
    ///     - Identify trending stocks with visualization
    pub async fn top_change(&self, date: &str, filter_column: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<TopChangeResponse, crate::error::InvezgoError> {
        let path = "/analysis/top/change".to_string();
        let query = [
            ("date", Some(date.to_string())),
            ("filter_column", filter_column.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar saham dengan akumulasi dan distribusi asing terbesar pada hari tertentu, dilengkapi dengan data grafik 5 hari ke belakang.
    /// 
    ///     Fitur:
    ///     - Top Akumulasi: Saham dengan akumulasi asing tertinggi
    ///     - Top Distribusi: Saham dengan distribusi asing terbesar
    ///     - Graph: Data persentase foreign dari total value 5 hari ke belakang untuk setiap saham
    /// 
    ///     Jenis Data: Cross Section + Time Series (data harian + grafik)
    /// 
    ///     Update: End of Day (EOD) setiap hari setelah jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Catatan:
    ///     - Data tidak tersedia pada hari libur atau weekend
    ///     - Gunakan tanggal hari kerja untuk hasil terbaik
    ///     - Setiap saham dilengkapi dengan data grafik 5 hari
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa pergerakan asing dengan tren
    ///     - Identifikasi saham favorit asing dengan visualisasi
    ///     - Analisa sentimen pasar dengan mini chart
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of stocks with the largest foreign accumulation and distribution on a specific day, complete with 5-day historical graph data.
    /// 
    ///     Features:
    ///     - Top Accumulation: Stocks with the highest foreign accumulation
    ///     - Top Distribution: Stocks with the largest foreign distribution
    ///     - Graph: 5-day historical foreign percentage data for each stock
    /// 
    ///     Data Type: Cross Section + Time Series (daily data + graph)
    /// 
    ///     Update: End of Day (EOD) every day after 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Notes:
    ///     - Data not available on holidays or weekends
    ///     - Use business days for best results
    ///     - Each stock includes 5-day graph data
    /// 
    ///     Usage Examples:
    ///     - Foreign movement analysis with trends
    ///     - Identify foreign favorite stocks with visualization
    ///     - Market sentiment analysis with mini charts
    pub async fn top_foreign(&self, date: &str, filter_column: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<TopForeignResponse, crate::error::InvezgoError> {
        let path = "/analysis/top/foreign".to_string();
        let query = [
            ("date", Some(date.to_string())),
            ("filter_column", filter_column.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar saham dengan akumulasi dan distribusi bandarmologi terbesar pada hari tertentu.
    /// 
    ///     Fitur:
    ///     - Top Akumulasi: Saham dengan akumulasi bandarmologi terbesar
    ///     - Top Distribusi: Saham dengan distribusi bandarmologi terbesar
    ///     - Data akumulasi dan distribusi
    /// 
    ///     Jenis Data: Cross Section (data harian)
    /// 
    ///     Update: End of Day (EOD) setiap hari setelah jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Catatan:
    ///     - Data tidak tersedia pada hari libur atau weekend
    ///     - Gunakan tanggal hari kerja untuk hasil terbaik
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa bandarmologi
    ///     - Identifikasi saham dengan akumulasi kuat
    ///     - Identifikasi saham dengan distribusi kuat
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of stocks with the largest bandarmology accumulation and distribution on a specific day.
    /// 
    ///     Features:
    ///     - Top Accumulation: Stocks with the largest bandarmology accumulation
    ///     - Top Distribution: Stocks with the largest bandarmology distribution
    ///     - Accumulation and distribution data
    /// 
    ///     Data Type: Cross Section (daily data)
    /// 
    ///     Update: End of Day (EOD) every day after 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Notes:
    ///     - Data not available on holidays or weekends
    ///     - Use business days for best results
    /// 
    ///     Usage Examples:
    ///     - Bandarmology analysis
    ///     - Identify stocks with strong accumulation
    ///     - Identify stocks with strong distribution
    pub async fn top_accumulation(&self, date: &str, filter_column: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<TopAccumulationResponse, crate::error::InvezgoError> {
        let path = "/analysis/top/accumulation".to_string();
        let query = [
            ("date", Some(date.to_string())),
            ("filter_column", filter_column.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar saham dengan akumulasi dan distribusi ritel terbesar pada hari tertentu.
    /// 
    ///     Fitur:
    ///     - Top Akumulasi: Saham dengan akumulasi ritel tertinggi
    ///     - Top Distribusi: Saham dengan distribusi ritel terbesar
    ///     - Data persentase ritel dari total volume
    /// 
    ///     Jenis Data: Cross Section (data harian)
    /// 
    ///     Update: End of Day (EOD) setiap hari setelah jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Catatan:
    ///     - Data tidak tersedia pada hari libur atau weekend
    ///     - Gunakan tanggal hari kerja untuk hasil terbaik
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa pergerakan ritel
    ///     - Identifikasi saham favorit ritel
    ///     - Analisa sentimen pasar ritel
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of stocks with the largest retail accumulation and distribution on a specific day.
    /// 
    ///     Features:
    ///     - Top Accumulation: Stocks with the highest retail accumulation
    ///     - Top Distribution: Stocks with the largest retail distribution
    ///     - Retail percentage data from total volume
    /// 
    ///     Data Type: Cross Section (daily data)
    /// 
    ///     Update: End of Day (EOD) every day after 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Notes:
    ///     - Data not available on holidays or weekends
    ///     - Use business days for best results
    /// 
    ///     Usage Examples:
    ///     - Retail movement analysis
    ///     - Identify retail favorite stocks
    ///     - Retail market sentiment analysis
    pub async fn top_ritel(&self, date: &str, filter_column: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<TopRitelResponse, crate::error::InvezgoError> {
        let path = "/analysis/top/ritel".to_string();
        let query = [
            ("date", Some(date.to_string())),
            ("filter_column", filter_column.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data intraday chart untuk saham dengan tanggal terakhir.
    /// 
    ///     Data yang Disediakan:
    ///     - Data intraday chart dengan data ringkas
    ///     - Data time series dengan panjang terbatas
    ///     - Data chart-ready untuk visualisasi intraday
    ///     - Data ringkas untuk tampilan cepat
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Contoh Penggunaan:
    ///     - Intraday chart untuk workspace
    ///     - Tampilan ringkas harga saham
    ///     - Quick overview chart
    ///     - Mobile-friendly chart data
    ///     - 1D Chart
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get intraday chart data for a stock with the latest date.
    /// 
    ///     Data Provided:
    ///     - Intraday chart data with summary data
    ///     - Time series data with limited length
    ///     - Chart-ready data for intraday visualization
    ///     - Summary data for quick display
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Usage Examples:
    ///     - Intraday chart for workspace
    ///     - Stock price summary display
    ///     - Quick overview chart
    ///     - Mobile-friendly chart data
    ///     - 1D Chart
    pub async fn get_intraday_chart(&self, code: &str, market: &str) -> Result<Vec<GetIntradayChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/intraday/{}", code);
        let query = [
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data order book untuk saham dengan tanggal terakhir atau data historis pada tanggal/waktu tertentu.
    /// 
    ///     Mode Pengambilan Data:
    ///     - **Realtime**: Tanpa parameter date/time - menggunakan data realtime saat jam trading atau snapshot terbaru
    ///     - **Historis**: Dengan parameter date dan time - mengambil data order book pada timestamp tertentu
    /// 
    ///     Data yang Disediakan:
    ///     - Data order book dengan data ringkas (bid/offer levels)
    ///     - Data cross section dengan panjang terbatas
    ///     - Data table-ready untuk visualisasi order book
    ///     - Data ringkas untuk tampilan cepat
    /// 
    ///     Parameter Historis:
    ///     - date: Tanggal dalam format YYYY-MM-DD (opsional, minimal 2026-03-25)
    ///     - time: Waktu dalam format HH:MM dengan presisi menit (opsional)
    ///     - Catatan: date dan time HARUS diisi bersamaan atau kedua-duanya tidak diisi
    /// 
    ///     Jenis Data: Cross Section
    /// 
    ///     Update: Real-time (tanpa date/time) atau Historical (dengan date/time)
    /// 
    ///     Contoh Penggunaan:
    ///     - Order book realtime untuk workspace
    ///     - Analisis order book historis pada waktu tertentu
    ///     - Tampilan ringkas harga saham
    ///     - Quick overview order book
    ///     - Mobile-friendly order book data
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get order book data for a stock with the latest date or historical data at specific date/time.
    /// 
    ///     Data Retrieval Modes:
    ///     - **Realtime**: Without date/time parameters - uses realtime data during trading hours or latest snapshot
    ///     - **Historical**: With date and time parameters - retrieves order book data at specific timestamp
    /// 
    ///     Data Provided:
    ///     - Order book data with summary data (bid/offer levels)
    ///     - Cross section data with limited length
    ///     - Table-ready data for order book visualization
    ///     - Summary data for quick display
    /// 
    ///     Historical Parameters:
    ///     - date: Date in YYYY-MM-DD format (optional, minimum 2026-03-25)
    ///     - time: Time in HH:MM format with minute precision (optional)
    ///     - Note: date and time MUST be filled together or both empty
    /// 
    ///     Data Type: Cross Section
    /// 
    ///     Update: Real-time (without date/time) or Historical (with date/time)
    /// 
    ///     Usage Examples:
    ///     - Realtime order book for workspace
    ///     - Historical order book analysis at specific time
    ///     - Stock price summary display
    ///     - Quick overview order book
    ///     - Mobile-friendly order book data
    pub async fn get_order_book(&self, code: &str, market: &str, date: Option<&str>, time: Option<&str>) -> Result<GetOrderBookResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/order-book/{}", code);
        let query = [
            ("market", Some(market.to_string())),
            ("date", date.map(|v| v.to_string())),
            ("time", time.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data harga intraday untuk saham.
    ///     - Tanpa query `date`: data terbaru (real-time/hybrid)
    ///     - Dengan query `date` (YYYY-MM-DD): data historis pada tanggal tersebut
    /// 
    ///     Data yang Disediakan:
    ///     - Data harga intraday dengan data ringkas
    ///     - Data harga intraday dengan panjang terbatas
    ///     - Data harga intraday ringkas untuk tampilan cepat
    /// 
    ///     Jenis Data: Cross Section
    /// 
    ///     Update: Real-time (tanpa date) atau Historical (dengan date)
    /// 
    ///     Contoh Penggunaan:
    ///     - Intraday data untuk workspace
    ///     - Tampilan ringkas harga saham
    ///     - Quick overview intraday data
    ///     - Mobile-friendly intraday data
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get intraday stock data.
    ///     - Without `date` query: latest data (real-time/hybrid)
    ///     - With `date` query (YYYY-MM-DD): historical data for the selected date
    /// 
    ///     Data Provided:
    ///     - Intraday data with summary data
    ///     - Cross section data with limited length
    ///     - Table-ready data for intraday visualization
    ///     - Summary data for quick display
    /// 
    ///     Data Type: Cross Section
    /// 
    ///     Update: Real-time (without date) or Historical (with date)
    /// 
    ///     Usage Examples:
    ///     - Intraday data for workspace
    ///     - Stock price summary display
    ///     - Quick overview intraday data
    ///     - Mobile-friendly intraday data
    pub async fn get_intraday_data(&self, code: &str, market: &str, date: Option<&str>) -> Result<GetIntradayDataResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/intraday-data/{}", code);
        let query = [
            ("market", Some(market.to_string())),
            ("date", date.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data harga intraday untuk index dengan tanggal terakhir.
    /// 
    ///     Data yang Disediakan:
    ///     - Data harga intraday dengan data ringkas
    ///     - Data harga intraday dengan panjang terbatas
    ///     - Data harga intraday ringkas untuk tampilan cepat
    /// 
    ///     Jenis Data: Cross Section
    /// 
    ///     Update: Real-time
    /// 
    ///     Contoh Penggunaan:
    ///     - Intraday data untuk workspace
    ///     - Tampilan ringkas harga saham
    ///     - Quick overview intraday data
    ///     - Mobile-friendly intraday data
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get intraday data for a stock with the latest date.
    /// 
    ///     Data Provided:
    ///     - Intraday data with summary data
    ///     - Cross section data with limited length
    ///     - Table-ready data for intraday visualization
    ///     - Summary data for quick display
    /// 
    ///     Data Type: Cross Section
    /// 
    ///     Update: Real-time
    /// 
    ///     Usage Examples:
    ///     - Intraday data for workspace
    ///     - Index price summary display
    ///     - Quick overview intraday data
    ///     - Mobile-friendly intraday data
    pub async fn get_intraday_index(&self, code: &str, market: &str, date: Option<&str>) -> Result<GetIntradayIndexResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/intraday-index/{}", code);
        let query = [
            ("market", Some(market.to_string())),
            ("date", date.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data grafik harga saham lengkap dengan data OHLCV pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Data harga OHLCV (Open, High, Low, Close, Volume)
    ///     - Data time series dalam periode tertentu
    ///     - Informasi transaksi harian pada periode tertentu
    /// 
    ///     Jenis Data: Time Series (data historis)
    /// 
    ///     Update: Realtime
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Chart
    ///     - Analisa volume dan price action
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get complete stock price chart data with OHLCV data for a specific period.
    /// 
    ///     Data Provided:
    ///     - OHLCV price data (Open, High, Low, Close, Volume)
    ///     - Time series data within specific period
    ///     - Daily transaction information for specific period
    /// 
    ///     Data Type: Time Series (historical data)
    /// 
    ///     Update: Realtime
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Usage Examples:
    ///     - Chart
    ///     - Volume and price action analysis
    pub async fn get_advance_chart(&self, code: &str, from: &str, to: &str) -> Result<Vec<GetAdvanceChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/chart/stock/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data grafik harga index lengkap dengan data OHLCV pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Data harga OHLCV (Open, High, Low, Close, Volume)
    ///     - Data time series dalam periode tertentu
    ///     - Informasi transaksi harian pada periode tertentu
    /// 
    ///     Jenis Data: Time Series (data historis)
    /// 
    ///     Update: Realtime
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Chart Index (IHSG, LQ45, IDX30, dll)
    ///     - Analisa volume dan price action index
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get complete index price chart data with OHLCV data for a specific period.
    /// 
    ///     Data Provided:
    ///     - OHLCV price data (Open, High, Low, Close, Volume)
    ///     - Time series data within specific period
    ///     - Daily transaction information for specific period
    /// 
    ///     Data Type: Time Series (historical data)
    /// 
    ///     Update: Realtime
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Usage Examples:
    ///     - Index Chart (COMPOSITE, LQ45, IDX30, etc)
    ///     - Volume and price action analysis for index
    pub async fn get_index_chart(&self, code: &str, from: &str, to: &str) -> Result<Vec<GetIndexChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/chart/index/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data grafik multi-timeframe untuk saham atau index.
    /// 
    ///     **Mendukung:**
    ///     - Saham (contoh: BBCA, BBRI, TLKM)
    ///     - Index (contoh: COMPOSITE, LQ45, IDX30)
    /// 
    ///     Timeframe yang Tersedia:
    ///     - **1**: 1 menit
    ///     - **5**: 5 menit
    ///     - **15**: 15 menit
    ///     - **30**: 30 menit
    ///     - **60**: 1 jam
    ///     - **D**: Harian (Daily)
    ///     - **W**: Mingguan (Weekly)
    ///     - **M**: Bulanan (Monthly)
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Realtime
    /// 
    ///     Contoh Penggunaan:
    ///     - Chart multi-timeframe
    ///     - Analisa teknikal dengan berbagai timeframe
    /// 
    ///     ---
    /// 
    ///     Get multi-timeframe chart data for stocks or indexes.
    /// 
    ///     **Supports:**
    ///     - Stocks (e.g., BBCA, BBRI, TLKM)
    ///     - Indexes (e.g., COMPOSITE, LQ45, IDX30)
    /// 
    ///     Available Timeframes:
    ///     - **1**: 1 minute
    ///     - **5**: 5 minutes
    ///     - **15**: 15 minutes
    ///     - **30**: 30 minutes
    ///     - **60**: 1 hour
    ///     - **D**: Daily
    ///     - **W**: Weekly
    ///     - **M**: Monthly
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Realtime
    /// 
    ///     Usage Examples:
    ///     - Multi-timeframe chart
    ///     - Technical analysis with various timeframes
    pub async fn get_multi_time_chart(&self, code: &str, from: &str, to: &str, timeframe: &str) -> Result<Vec<GetMultiTimeChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/chart/multi-time/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("timeframe", Some(timeframe.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data grafik dengan indikator teknikal untuk analisis harga saham pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Berbagai jenis indikator custom (BDM, FOREIGN, RATIO, RITEL ) pada periode tertentu
    ///     - Data time series dengan indikator
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa teknikal saham
    ///     - Identifikasi sinyal trading
    ///     - Analisa trend dengan indikator
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get chart data with technical indicators for stock price analysis in a specific period.
    /// 
    ///     Data Provided:
    ///     - Various types of indicators (BDM, FOREIGN, RATIO, RITEL) for specific period
    ///     - Time series data with indicators
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Usage Examples:
    ///     - Stock technical analysis
    ///     - Trading signal identification
    ///     - Trend analysis with indicators
    pub async fn get_indicator_chart(&self, code: &str, indicator: &str, from: &str, to: &str) -> Result<Vec<GetIndicatorChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/chart/stock/{}/{}", indicator, code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data kepemilikan saham perusahaan dalam periode waktu tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Kepemilikan saham
    ///     - Data time series perubahan kepemilikan
    ///     - Trend pertumbuhan/penurunan kepemilikan
    ///     - Data historis kepemilikan saham
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: Bulanan sesuai laporan masing-masing perusahaan
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa popularitas saham
    ///     - Due diligence investasi
    ///     - Analisa sentimen pasar
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get company stock ownership data for a specific period.
    /// 
    ///     Data Provided:
    ///     - Stock ownership
    ///     - Time series data of ownership changes
    ///     - Growth/decline trend of ownership
    ///     - Historical stock ownership data
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: Monthly according to each company's report
    /// 
    ///     Usage Examples:
    ///     - Stock popularity analysis
    ///     - Investment due diligence
    ///     - Market sentiment analysis
    pub async fn shareholder_detail(&self, code: &str, name: Option<&str>) -> Result<Vec<ShareholderDetailResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder-detail/{}", code);
        let query = [
            ("name", name.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data kepemilikan saham perusahaan berdasarkan kode saham atau nama pemegang saham.
    /// 
    ///     Data yang Disediakan:
    ///     - Kepemilikan saham (pemilik > 1%)
    ///     - Informasi detail pemegang saham (tipe, status, domisili)
    ///     - Jumlah saham dan persentase kepemilikan
    /// 
    ///     Parameter:
    ///     - code: kode saham (opsional)
    ///     - name: nama pemegang saham (opsional)
    /// 
    ///     Validasi:
    ///     - Minimal salah satu dari code atau name harus diisi
    /// 
    ///     Jenis Data: Cross Section
    /// 
    ///     Update: Bulanan sesuai laporan masing-masing perusahaan
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa struktur kepemilikan saham
    ///     - Due diligence investasi
    ///     - Identifikasi pemegang saham mayoritas
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get company stock ownership data based on stock code or shareholder name.
    /// 
    ///     Data Provided:
    ///     - Stock ownership (> 1% owners)
    ///     - Shareholder details (type, status, domicile)
    ///     - Share count and ownership percentage
    /// 
    ///     Parameters:
    ///     - code: stock code (optional)
    ///     - name: shareholder name (optional)
    /// 
    ///     Validation:
    ///     - At least one of code or name must be provided
    /// 
    ///     Data Type: Cross Section
    /// 
    ///     Update: Monthly according to each company's report
    /// 
    ///     Usage Examples:
    ///     - Stock ownership structure analysis
    ///     - Investment due diligence
    ///     - Major shareholder identification
    pub async fn shareholder_one_detail(&self, code: Option<&str>, name: Option<&str>) -> Result<Vec<ShareholderOneDetailResponse>, crate::error::InvezgoError> {
        let path = "/analysis/shareholder-detail-one".to_string();
        let query = [
            ("code", code.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data konsentrasi kepemilikan saham tinggi untuk semua emiten.
    /// 
    ///     Data yang Disediakan:
    ///     - Kode saham
    ///     - Tanggal laporan
    ///     - Persentase kepemilikan konsentrasi tinggi
    /// 
    ///     Jenis Data: Cross Section
    /// 
    ///     Update: Bulanan sesuai laporan masing-masing perusahaan
    /// 
    ///     Contoh Penggunaan:
    ///     - Identifikasi saham dengan konsentrasi kepemilikan tinggi
    ///     - Analisis struktur kepemilikan pasar
    ///     - Screening untuk investasi
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get high concentration stock ownership data for all issuers.
    /// 
    ///     Data Provided:
    ///     - Stock code
    ///     - Report date
    ///     - High concentration ownership percentage
    /// 
    ///     Data Type: Cross Section
    /// 
    ///     Update: Monthly according to each company's report
    /// 
    ///     Usage Examples:
    ///     - Identify stocks with high ownership concentration
    ///     - Market ownership structure analysis
    ///     - Screening for investment
    pub async fn shareholder_high(&self) -> Result<Vec<ShareholderHighResponse>, crate::error::InvezgoError> {
        let path = "/analysis/shareholder/high".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data jumlah investor/pemegang saham perusahaan dalam periode waktu tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Jumlah total pemegang saham
    ///     - Data time series perubahan jumlah investor
    ///     - Trend pertumbuhan/penurunan investor
    ///     - Data historis jumlah pemegang saham
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: Bulanan sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa popularitas saham
    ///     - Due diligence investasi
    ///     - Analisa sentimen pasar
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get the number of investors/shareholders of a company for a specific period.
    /// 
    ///     Data Provided:
    ///     - Total number of shareholders
    ///     - Time series data of investor count changes
    ///     - Investor growth/decline trend
    ///     - Historical shareholder count data
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: Monthly according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Stock popularity analysis
    ///     - Investment due diligence
    ///     - Market sentiment analysis
    pub async fn shareholder_number(&self, code: &str) -> Result<Vec<ShareholderNumberResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder/number/{}", code);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data graf relasi pemegang saham secara dua arah berdasarkan snapshot kepemilikan saham terbaru.
    /// 
    ///     Data yang Disediakan:
    ///     - Nodes: saham dan entitas/pemegang saham
    ///     - Edges: relasi kepemilikan saham -> pemegang saham
    ///     - Struktur kepemilikan lintas perusahaan
    ///     - Data anak perusahaan (subsidiary)
    /// 
    ///     Parameter:
    ///     - code: kode saham sebagai titik awal pencarian
    ///     - name: nama pemegang saham sebagai titik awal
    ///     - depth: kedalaman graph yang dibangun
    ///     - max_nodes: batas maksimum jumlah node
    ///     - min_percentage: filter persentase kepemilikan minimum
    ///     - source_type: sumber data (idx, ksei, ubo, subsidiary)
    /// 
    ///     Jenis Data: Cross Section
    /// 
    ///     Update: Harian sesuai ketersediaan data
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisis struktur kepemilikan grup perusahaan
    ///     - Due diligence investasi
    ///     - Identifikasi beneficial owner
    ///     - Analisis jaringan keterkaitan entitas bisnis
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get bidirectional shareholder relation graph data based on the latest stock ownership snapshots.
    /// 
    ///     Data Provided:
    ///     - Nodes: stocks and entities/shareholders
    ///     - Edges: ownership relations from stock -> shareholder
    ///     - Cross-company ownership structure
    ///     - Subsidiary data
    /// 
    ///     Parameters:
    ///     - code: stock code as the starting point
    ///     - name: shareholder name as the starting point
    ///     - depth: depth of the graph to build
    ///     - max_nodes: maximum number of nodes limit
    ///     - min_percentage: minimum ownership percentage filter
    ///     - source_type: data source (idx, ksei, ubo, subsidiary)
    /// 
    ///     Data Type: Cross Section
    /// 
    ///     Update: Daily according to data availability
    /// 
    ///     Usage Examples:
    ///     - Corporate group ownership structure analysis
    ///     - Investment due diligence
    ///     - Beneficial owner identification
    ///     - Business entity network analysis
    pub async fn shareholder_relation(&self, code: Option<&str>, name: Option<&str>, depth: Option<&str>, max_nodes: Option<&str>, neighbors: Option<&str>, min_percentage: Option<&str>) -> Result<ShareholderRelationResponse, crate::error::InvezgoError> {
        let path = "/analysis/shareholder/relation".to_string();
        let query = [
            ("code", code.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("depth", depth.map(|v| v.to_string())),
            ("max_nodes", max_nodes.map(|v| v.to_string())),
            ("neighbors", neighbors.map(|v| v.to_string())),
            ("min_percentage", min_percentage.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data komposisi kepemilikan saham perusahaan berdasarkan pemegang saham terbesar pada tanggal terbaru.
    /// 
    ///     Data yang Disediakan:
    ///     - Pemegang saham pengendali (controlling shareholder)
    ///     - Pihak afiliasi dan direksi
    ///     - Masyarakat umum (public shareholders)
    ///     - Persentase kepemilikan masing-masing pihak
    ///     - Badge/jabatan (PENGENDALI, DIREKSI, KOMISARIS)
    /// 
    ///     Jenis Data: Cross Section (data snapshot terbaru)
    /// 
    ///     Update: Bulanan sesuai laporan masing-masing perusahaan
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa struktur kepemilikan
    ///     - Identifikasi pemegang saham mayoritas
    ///     - Due diligence investasi
    ///     - Analisa corporate governance
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get company stock ownership composition data based on the largest shareholders on the latest date.
    /// 
    ///     Data Provided:
    ///     - Controlling shareholder
    ///     - Affiliated parties and directors
    ///     - Public shareholders
    ///     - Ownership percentage of each party
    ///     - Badge/position (CONTROLLER, DIRECTOR, COMMISSIONER)
    /// 
    ///     Data Type: Cross Section (latest snapshot data)
    /// 
    ///     Update: Monthly according to each company's report
    /// 
    ///     Usage Examples:
    ///     - Ownership structure analysis
    ///     - Majority shareholder identification
    ///     - Investment due diligence
    ///     - Corporate governance analysis
    pub async fn shareholder(&self, code: &str) -> Result<Vec<ShareholderResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder/{}", code);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data pemegang saham berdasarkan klasifikasi KSEI (investor asing dan domestik dalam berbagai grup) pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Klasifikasi investor asing dan domestik
    ///     - Grup investor (IS, CP, PF, IB, ID, MF, SC, FD, OT)
    ///     - Data time series perubahan kepemilikan saham
    ///     - Persentase kepemilikan per grup investor
    /// 
    ///     Grup Investor:
    ///     - SC: Sekuritas
    ///     - ID: Individu
    ///     - CP: Korporasi
    ///     - OT: Lainnya
    ///     - IB: Institusi Finansial
    ///     - IS: Asuransi
    ///     - FD: Yayasan
    ///     - MF: Reksadana
    ///     - PF: Dana Pensiun
    /// 
    ///     - Foreign: Investor Asing
    ///     - Local: Investor Domestik
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: Tanggal 02 setiap bulan
    /// 
    ///     Data Awal: 2009-03-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa komposisi investor
    ///     - Analisa flow asing vs domestik
    ///     - Identifikasi tipe investor dominan
    ///     - Analisa sentimen pasar
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get shareholder data based on KSEI classification (foreign and domestic investors in various groups) for a specific period.
    /// 
    ///     Data Provided:
    ///     - Foreign and domestic investor classification
    ///     - Investor groups (IS, CP, PF, IB, ID, MF, SC, FD, OT)
    ///     - Time series data of ownership changes
    ///     - Ownership percentage per investor group
    /// 
    ///     Investor Group:
    ///     - SC: Broker
    ///     - ID: Individual
    ///     - CP: Corporate
    ///     - OT: Other
    ///     - IB: Financial Institution
    ///     - IS: Insurance
    ///     - FD: Foundation
    ///     - MF: Mutual Fund
    ///     - PF: Pension Fund
    /// 
    ///     Investor Classification:
    ///     - Foreign: Investor Asing
    ///     - Local: Investor Domestik
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: 2nd of every month
    /// 
    ///     Initial Data: 2009-03-01
    /// 
    ///     Usage Examples:
    ///     - Investor composition analysis
    ///     - Foreign vs domestic flow analysis
    ///     - Dominant investor type identification
    ///     - Market sentiment analysis
    pub async fn shareholder_ksei(&self, code: &str, range: &str) -> Result<Vec<ShareholderKseiResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder/ksei/{}", code);
        let query = [
            ("range", Some(range.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data lengkap pemegang saham dengan seluruh klasifikasi investor pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Seluruh record time series kepemilikan saham (sesuai range)
    ///     - Klasifikasi lengkap investor (40 kategori)
    ///     - Data total kepemilikan per periode
    /// 
    ///     Parameter:
    ///     - range: Jumlah data yang diambil (1-36 bulan)
    /// 
    ///     Klasifikasi Investor:
    ///     - BK: Bank
    ///     - GV: Pemerintah (Government)
    ///     - PE: Private Equity
    ///     - TB: Trustee Bank
    ///     - VC: Venture Capital
    ///     - PB: Private Bank
    ///     - EF: Exchange Traded Funds (ETF)
    ///     - IM: Manajer Investasi
    ///     - IA: Investment Advisors
    ///     - BR: Perusahaan Sekuritas (Brokerage Firms)
    ///     - HF: Hedge Fund
    ///     - SW: Sovereign Wealth Fund
    ///     - CM: Lembaga Pendukung Pasar Modal
    ///     - CV: Commanditaire Vennootschap (CV)
    ///     - FM: Firma (Firm)
    ///     - SA: Agen Penjual Reksa Dana
    ///     - PP: Peer to Peer Lending
    ///     - PS: Permanent Establishment
    ///     - SP: Sole Proprietorship (Usaha Perorangan)
    ///     - CR: Korporasi (Corporate)
    ///     - AS: Asosiasi / Organisasi Sosial
    ///     - SO: Badan Usaha Milik Negara (BUMN)
    ///     - CB: Bank Sentral
    ///     - OC: Perusahaan Milik Negara
    ///     - DC: Keuskupan (Diocese)
    ///     - CN: Konferensi (Conference)
    ///     - CG: Kongregasi (Congregation)
    ///     - CP: Koperasi
    ///     - IO: Organisasi Internasional
    ///     - PL: Partai Politik
    ///     - PT: Partnership (Persekutuan)
    ///     - ED: Lembaga Pendidikan
    ///     - MF: Reksa Dana (Mutual Funds)
    ///     - SC: Perusahaan Sekuritas
    ///     - PF: Dana Pensiun
    ///     - IB: Lembaga Finansial
    ///     - IS: Asuransi
    ///     - FD: Yayasan (Foundation)
    ///     - IN: Individu
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa komposisi investor lengkap
    ///     - Tracking perubahan kepemilikan
    ///     - Identifikasi investor dominan
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get complete shareholder data with all investor classifications for a specific period.
    /// 
    ///     Data Provided:
    ///     - Complete time series ownership records (based on range)
    ///     - Full investor classifications (40 categories)
    ///     - Total ownership data per period
    /// 
    ///     Parameter:
    ///     - range: Number of records to retrieve (1-36 months)
    /// 
    ///     Investor Classification:
    ///     - BK: Bank
    ///     - GV: Government
    ///     - PE: Private Equity
    ///     - TB: Trustee Bank
    ///     - VC: Venture Capital
    ///     - PB: Private Bank
    ///     - EF: Exchange Traded Funds
    ///     - IM: Investment Manager
    ///     - IA: Investment Advisors
    ///     - BR: Brokerage Firms
    ///     - HF: Hedge Fund
    ///     - SW: Sovereign Wealth Fund
    ///     - CM: Capital Market Supporting Institutions and Professions
    ///     - CV: Commanditaire Vennootschap (CV)
    ///     - FM: Firm
    ///     - SA: Investment Fund Selling Agent
    ///     - PP: Peer to Peer Lending
    ///     - PS: Permanent Establishment
    ///     - SP: Sole Proprietorship
    ///     - CR: Corporate
    ///     - AS: Association / Social Organizations
    ///     - SO: State Owned Enterprises
    ///     - CB: Central Bank
    ///     - OC: State Owned Company
    ///     - DC: Diocese
    ///     - CN: Conference
    ///     - CG: Congregation
    ///     - CP: Cooperatives
    ///     - IO: International Organization
    ///     - PL: Political Parties
    ///     - PT: Partnership
    ///     - ED: Educational Institution
    ///     - MF: Mutual Funds
    ///     - SC: Securities Company
    ///     - PF: Pension Funds
    ///     - IB: Financial Institutional
    ///     - IS: Insurance
    ///     - FD: Foundation
    ///     - IN: Individual
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Usage Examples:
    ///     - Complete investor composition analysis
    ///     - Ownership change tracking
    ///     - Dominant investor identification
    pub async fn shareholder_classification(&self, code: &str, range: &str) -> Result<Vec<ShareholderClassificationResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder/classification/{}", code);
        let query = [
            ("range", Some(range.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data terbaru pemegang saham dengan klasifikasi investor dalam bentuk tabel.
    /// 
    ///     Data yang Disediakan:
    ///     - Data terbaru kepemilikan saham
    ///     - Klasifikasi investor (40 kategori)
    ///     - Total kepemilikan
    /// 
    ///     Klasifikasi Investor:
    ///     - BK: Bank
    ///     - GV: Pemerintah (Government)
    ///     - PE: Private Equity
    ///     - TB: Trustee Bank
    ///     - VC: Venture Capital
    ///     - PB: Private Bank
    ///     - EF: Exchange Traded Funds (ETF)
    ///     - IM: Manajer Investasi
    ///     - IA: Investment Advisors
    ///     - BR: Perusahaan Sekuritas (Brokerage Firms)
    ///     - HF: Hedge Fund
    ///     - SW: Sovereign Wealth Fund
    ///     - CM: Lembaga Pendukung Pasar Modal
    ///     - CV: Commanditaire Vennootschap (CV)
    ///     - FM: Firma (Firm)
    ///     - SA: Agen Penjual Reksa Dana
    ///     - PP: Peer to Peer Lending
    ///     - PS: Permanent Establishment
    ///     - SP: Sole Proprietorship (Usaha Perorangan)
    ///     - CR: Korporasi (Corporate)
    ///     - AS: Asosiasi / Organisasi Sosial
    ///     - SO: Badan Usaha Milik Negara (BUMN)
    ///     - CB: Bank Sentral
    ///     - OC: Perusahaan Milik Negara
    ///     - DC: Keuskupan (Diocese)
    ///     - CN: Konferensi (Conference)
    ///     - CG: Kongregasi (Congregation)
    ///     - CP: Koperasi
    ///     - IO: Organisasi Internasional
    ///     - PL: Partai Politik
    ///     - PT: Partnership (Persekutuan)
    ///     - ED: Lembaga Pendidikan
    ///     - MF: Reksa Dana (Mutual Funds)
    ///     - SC: Perusahaan Sekuritas
    ///     - PF: Dana Pensiun
    ///     - IB: Lembaga Finansial
    ///     - IS: Asuransi
    ///     - FD: Yayasan (Foundation)
    ///     - IN: Individu
    /// 
    ///     Jenis Data: Cross Section (latest)
    /// 
    ///     Contoh Penggunaan:
    ///     - Quick view komposisi investor terbaru
    ///     - Dashboard display
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get latest shareholder data with investor classifications in table format.
    /// 
    ///     Data Provided:
    ///     - Latest ownership data
    ///     - 40 investor classifications
    ///     - Total ownership
    /// 
    ///     Investor Classification:
    ///     - BK: Bank
    ///     - GV: Government
    ///     - PE: Private Equity
    ///     - TB: Trustee Bank
    ///     - VC: Venture Capital
    ///     - PB: Private Bank
    ///     - EF: Exchange Traded Funds
    ///     - IM: Investment Manager
    ///     - IA: Investment Advisors
    ///     - BR: Brokerage Firms
    ///     - HF: Hedge Fund
    ///     - SW: Sovereign Wealth Fund
    ///     - CM: Capital Market Supporting Institutions and Professions
    ///     - CV: Commanditaire Vennootschap (CV)
    ///     - FM: Firm
    ///     - SA: Investment Fund Selling Agent
    ///     - PP: Peer to Peer Lending
    ///     - PS: Permanent Establishment
    ///     - SP: Sole Proprietorship
    ///     - CR: Corporate
    ///     - AS: Association / Social Organizations
    ///     - SO: State Owned Enterprises
    ///     - CB: Central Bank
    ///     - OC: State Owned Company
    ///     - DC: Diocese
    ///     - CN: Conference
    ///     - CG: Congregation
    ///     - CP: Cooperatives
    ///     - IO: International Organization
    ///     - PL: Political Parties
    ///     - PT: Partnership
    ///     - ED: Educational Institution
    ///     - MF: Mutual Funds
    ///     - SC: Securities Company
    ///     - PF: Pension Funds
    ///     - IB: Financial Institutional
    ///     - IS: Insurance
    ///     - FD: Foundation
    ///     - IN: Individual
    /// 
    ///     Data Type: Cross Section (latest)
    /// 
    ///     Usage Examples:
    ///     - Quick investor composition view
    ///     - Dashboard display
    pub async fn shareholder_classify_table(&self, code: &str) -> Result<ShareholderClassifyTableResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder/classify-table/{}", code);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan analisis detail aktivitas trading broker untuk saham tertentu dalam periode waktu tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Aktivitas buy/sell per broker
    ///     - Volume dan nilai transaksi
    ///     - Frekuensi trading
    ///     - Harga rata-rata buy/sell
    ///     - Klasifikasi investor (domestik/asing)
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa aktivitas saham
    ///     - Identifikasi broker dominan
    ///     - Analisa flow transaksi
    ///     - Analisa sentimen pasar
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get detailed trading broker activity analysis for a specific stock within a specific time period.
    /// 
    ///     Data Provided:
    ///     - Buy/sell activity per broker
    ///     - Volume and transaction value
    ///     - Trading frequency
    ///     - Average buy/sell price
    ///     - Investor classification (domestic/foreign)
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Stock activity analysis
    ///     - Dominant broker identification
    ///     - Transaction flow analysis
    ///     - Market sentiment analysis
    pub async fn summary_stock(&self, code: &str, from: &str, to: &str, investor: &str, market: &str) -> Result<Vec<SummaryStockResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/summary/stock/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("investor", Some(investor.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan analisis detail aktivitas trading broker tertentu untuk semua saham dalam periode waktu tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Aktivitas buy/sell per saham
    ///     - Volume dan nilai transaksi
    ///     - Frekuensi trading
    ///     - Harga rata-rata buy/sell
    ///     - Klasifikasi investor (domestik/asing)
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa performa broker
    ///     - Identifikasi saham favorit broker
    ///     - Analisa aktivitas trading broker
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get detailed trading activity analysis for a specific broker for all stocks within a specific time period.
    /// 
    ///     Data Provided:
    ///     - Buy/sell activity per stock
    ///     - Volume and transaction value
    ///     - Trading frequency
    ///     - Average buy/sell price
    ///     - Investor classification (domestic/foreign)
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Broker performance analysis
    ///     - Broker's favorite stocks identification
    ///     - Broker trading activity analysis
    pub async fn summary_broker(&self, code: &str, from: &str, to: &str, investor: &str, market: &str) -> Result<Vec<SummaryBrokerResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/summary/broker/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("investor", Some(investor.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data pergerakan harga indeks sektoral atau saham dalam rentang waktu tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Jika base = COMPOSITE: Pergerakan harga close indeks sektoral (IDXENERGY, IDXBASIC, IDXINDUST, IDXNONCYC, IDXCYCLIC, IDXHEALTH, IDXFINANCE, IDXPROPERT, IDXTECHNO, IDXINFRA, IDXTRANS, COMPOSITE)
    ///     - Data time series untuk setiap indeks/saham
    ///     - Data dinormalisasi ke base 100 untuk perbandingan
    /// 
    ///     Parameter:
    ///     - **from**: Tanggal mulai (format: YYYY-MM-DD)
    ///     - **to**: Tanggal akhir (format: YYYY-MM-DD)
    ///     - **base**: Base index untuk filter (default: COMPOSITE). Pilihan: COMPOSITE, IDX30, IDX80, LQ45, IDXFINANCE, IDXENERGY, dll
    ///     - **limit**: Jumlah saham yang ditampilkan (optional, hanya untuk base selain COMPOSITE, max: 100)
    ///     - **filter**: Filter tambahan untuk pencarian kode/nama saham (optional, hanya untuk base selain COMPOSITE)
    ///     - **filter_column**: Kolom stock_prices untuk filter (optional, hanya untuk base selain COMPOSITE). Pilihan: change, value, volume, foreign, freq, bdm, ritel, ratio, open, high, low, close
    ///     - **filter_operator**: Operator perbandingan untuk filter (optional). Pilihan: <, >, =, >=, <=, !=
    ///     - **filter_value**: Nilai untuk perbandingan filter (optional). Contoh: 5 (untuk filter change > 5)
    /// 
    ///     Filter Stock (hanya untuk base selain COMPOSITE):
    ///     Anda dapat memfilter saham berdasarkan nilai kolom di tabel stock_prices pada tanggal terakhir dalam range.
    /// 
    ///     Contoh Filter:
    ///     - filter_column=change&filter_operator=>&filter_value=5 → Saham dengan change > 5%
    ///     - filter_column=value&filter_operator=>=&filter_value=1000000000 → Saham dengan value >= 1 Miliar
    ///     - filter_column=volume&filter_operator=>&filter_value=1000000 → Saham dengan volume > 1 Juta
    ///     - filter_column=foreign&filter_operator=>&filter_value=0 → Saham dengan net foreign buy > 0
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Setiap hari bursa setelah penutupan pasar
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Catatan:
    ///     - Ketika base = COMPOSITE: menampilkan data index prices (perilaku default)
    ///     - Ketika base diubah ke index lain (misal IDX30): menampilkan data stock prices dari saham yang termasuk dalam index tersebut
    ///     - Data dinormalisasi ke base 100 untuk memudahkan perbandingan performa
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisis rotasi sektor (base = COMPOSITE)
    ///     - Perbandingan performa antar sektor (base = COMPOSITE)
    ///     - Identifikasi sektor yang outperform/underperform (base = COMPOSITE)
    ///     - Analisis performa saham dalam index tertentu (base = IDX30, IDX80, dll)
    ///     - Filter saham berdasarkan index dan pencarian (base = IDX30, filter = BANK)
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get sectoral index or stock price movement data within a specific time range.
    /// 
    ///     Data Provided:
    ///     - If base = COMPOSITE: Sectoral index close price movement (IDXENERGY, IDXBASIC, IDXINDUST, IDXNONCYC, IDXCYCLIC, IDXHEALTH, IDXFINANCE, IDXPROPERT, IDXTECHNO, IDXINFRA, IDXTRANS, COMPOSITE)
    ///     - If base = IDX30, IDX80, etc: Stock price movement for stocks included in that index
    ///     - Time series data for each index/stock
    ///     - Data normalized to base 100 for comparison
    /// 
    ///     Parameters:
    ///     - **from**: Start date (format: YYYY-MM-DD)
    ///     - **to**: End date (format: YYYY-MM-DD)
    ///     - **base**: Base index for filtering (default: COMPOSITE). Options: COMPOSITE, IDX30, IDX80, LQ45, IDXFINANCE, IDXENERGY, etc
    ///     - **limit**: Number of stocks to display (optional, only for base other than COMPOSITE, max: 100)
    ///     - **filter**: Additional filter for stock code/name search (optional, only for base other than COMPOSITE)
    ///     - **filter_column**: Column from stock_prices to filter (optional, only for base other than COMPOSITE). Options: change, value, volume, foreign, freq, bdm, ritel, ratio, open, high, low, close
    ///     - **filter_operator**: Comparison operator (optional). Options: <, >, =, >=, <=, !=
    ///     - **filter_value**: Value for comparison (optional). Example: 5 (for filter change > 5)
    /// 
    ///     Stock Filter (only for base other than COMPOSITE):
    ///     You can filter stocks based on column values in the stock_prices table on the last date in the range.
    /// 
    ///     Filter Examples:
    ///     - filter_column=change&filter_operator=>&filter_value=5 → Stocks with change > 5%
    ///     - filter_column=value&filter_operator=>=&filter_value=1000000000 → Stocks with value >= 1 Billion
    ///     - filter_column=volume&filter_operator=>&filter_value=1000000 → Stocks with volume > 1 Million
    ///     - filter_column=foreign&filter_operator=>&filter_value=0 → Stocks with net foreign buy > 0
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Every trading day after market close
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Notes:
    ///     - When base = COMPOSITE: displays index prices data (default behavior)
    ///     - When base is changed to another index (e.g., IDX30): displays stock prices data from stocks included in that index
    ///     - Data normalized to base 100 to facilitate performance comparison
    /// 
    ///     Usage Examples:
    ///     - Sector rotation analysis (base = COMPOSITE)
    ///     - Cross-sector performance comparison (base = COMPOSITE)
    ///     - Identify outperforming/underperforming sectors (base = COMPOSITE)
    ///     - Analyze stock performance within specific index (base = IDX30, IDX80, etc)
    ///     - Filter stocks by index and search (base = IDX30, filter = BANK)
    pub async fn sector_stalker(&self, from: &str, to: &str, base: Option<&str>, limit: Option<&str>, filter: Option<&str>, filter_column: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<SectorStalkerResponse, crate::error::InvezgoError> {
        let path = "/analysis/stalker/sector".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("base", base.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("filter", filter.map(|v| v.to_string())),
            ("filter_column", filter_column.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data rotasi sektor untuk visualisasi pergerakan relatif indeks sektoral atau saham.
    /// 
    ///     Data yang Disediakan:
    ///     - Jika base = COMPOSITE: Pergerakan relatif indeks sektoral terhadap COMPOSITE
    ///     - Jika base = index lain: Pergerakan relatif saham dalam index tersebut
    ///     - Koordinat X dan Y untuk plotting chart
    ///     - Trail data harian atau mingguan untuk melihat pergerakan historis
    ///     - Klasifikasi kuadran (leading, weakening, lagging, improving)
    /// 
    ///     Parameter:
    ///     - **from**: Tanggal mulai (format: YYYY-MM-DD)
    ///     - **to**: Tanggal akhir (format: YYYY-MM-DD)
    ///     - **base**: Base index untuk filter (default: COMPOSITE). Pilihan: COMPOSITE, IDX30, IDX80, LQ45, IDXFINANCE, IDXENERGY, dll
    ///     - **length**: Periode perhitungan (default: 10, range: 5-50)
    ///     - **interval**: Interval perhitungan trail. Pilihan: daily, weekly (default: weekly)
    ///     - **tail**: Jumlah titik trail yang ditampilkan (default: 5, range: 1-52)
    ///     - **limit**: Jumlah saham yang ditampilkan (optional, hanya untuk base selain COMPOSITE, max: 50)
    ///     - **filter**: Filter tambahan untuk pencarian kode/nama saham (optional, hanya untuk base selain COMPOSITE)
    ///     - **filter_column**: Kolom stock_prices untuk filter (optional, hanya untuk base selain COMPOSITE). Pilihan: change, value, volume, foreign, freq, bdm, ritel, ratio, open, high, low, close
    ///     - **filter_operator**: Operator perbandingan untuk filter (optional). Pilihan: <, >, =, >=, <=, !=
    ///     - **filter_value**: Nilai untuk perbandingan filter (optional). Contoh: 5 (untuk filter change > 5)
    /// 
    ///     Filter Stock (hanya untuk base selain COMPOSITE):
    ///     Anda dapat memfilter saham berdasarkan nilai kolom di tabel stock_prices pada tanggal terakhir dalam range.
    /// 
    ///     Contoh Filter:
    ///     - filter_column=change&filter_operator=>&filter_value=5 → Saham dengan change > 5%
    ///     - filter_column=value&filter_operator=>=&filter_value=1000000000 → Saham dengan value >= 1 Miliar
    ///     - filter_column=volume&filter_operator=>&filter_value=1000000 → Saham dengan volume > 1 Juta
    ///     - filter_column=foreign&filter_operator=>&filter_value=0 → Saham dengan net foreign buy > 0
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Setiap hari bursa setelah penutupan pasar
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Catatan:
    ///     - Ketika base = COMPOSITE: menampilkan data indeks sektoral
    ///     - Ketika base diubah ke index lain (misal IDX30): menampilkan data saham dari index tersebut
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisis rotasi sektor (base = COMPOSITE)
    ///     - Identifikasi sektor yang outperform/underperform
    ///     - Analisis performa saham dalam index tertentu (base = IDX30, IDX80, dll)
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get sector rotation data for visualizing relative movement of sectoral indices or stocks.
    /// 
    ///     Data Provided:
    ///     - If base = COMPOSITE: Relative movement of sectoral indices against COMPOSITE
    ///     - If base = other index: Relative movement of stocks within that index
    ///     - X and Y coordinates for chart plotting
    ///     - Daily or weekly trail data for historical movement visualization
    ///     - Quadrant classification (leading, weakening, lagging, improving)
    /// 
    ///     Parameters:
    ///     - **from**: Start date (format: YYYY-MM-DD)
    ///     - **to**: End date (format: YYYY-MM-DD)
    ///     - **base**: Base index for filtering (default: COMPOSITE). Options: COMPOSITE, IDX30, IDX80, LQ45, IDXFINANCE, IDXENERGY, etc
    ///     - **length**: Calculation period (default: 10, range: 5-50)
    ///     - **interval**: Trail calculation interval. Options: daily, weekly (default: weekly)
    ///     - **tail**: Number of trail points to display (default: 5, range: 1-52)
    ///     - **limit**: Number of stocks to display (optional, only for base other than COMPOSITE, max: 50)
    ///     - **filter**: Additional filter for stock code/name search (optional, only for base other than COMPOSITE)
    ///     - **filter_column**: Column from stock_prices to filter (optional, only for base other than COMPOSITE). Options: change, value, volume, foreign, freq, bdm, ritel, ratio, open, high, low, close
    ///     - **filter_operator**: Comparison operator (optional). Options: <, >, =, >=, <=, !=
    ///     - **filter_value**: Value for comparison (optional). Example: 5 (for filter change > 5)
    /// 
    ///     Stock Filter (only for base other than COMPOSITE):
    ///     You can filter stocks based on column values in the stock_prices table on the last date in the range.
    /// 
    ///     Filter Examples:
    ///     - filter_column=change&filter_operator=>&filter_value=5 → Stocks with change > 5%
    ///     - filter_column=value&filter_operator=>=&filter_value=1000000000 → Stocks with value >= 1 Billion
    ///     - filter_column=volume&filter_operator=>&filter_value=1000000 → Stocks with volume > 1 Million
    ///     - filter_column=foreign&filter_operator=>&filter_value=0 → Stocks with net foreign buy > 0
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Every trading day after market close
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Notes:
    ///     - When base = COMPOSITE: displays sectoral index data
    ///     - When base is changed to another index (e.g., IDX30): displays stock data from that index
    /// 
    ///     Usage Examples:
    ///     - Sector rotation analysis (base = COMPOSITE)
    ///     - Identify outperforming/underperforming sectors
    ///     - Analyze stock performance within specific index (base = IDX30, IDX80, etc)
    pub async fn sector_rotation(&self, from: &str, to: &str, base: Option<&str>, length: Option<&str>, interval: Option<&str>, tail: Option<&str>, limit: Option<&str>, filter: Option<&str>, filter_column: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<SectorRotationResponse, crate::error::InvezgoError> {
        let path = "/analysis/sector/rotation".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("base", base.map(|v| v.to_string())),
            ("length", length.map(|v| v.to_string())),
            ("interval", interval.map(|v| v.to_string())),
            ("tail", tail.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("filter", filter.map(|v| v.to_string())),
            ("filter_column", filter_column.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Melacak aktivitas trading broker tertentu pada saham tertentu dalam format kalender harian.
    /// 
    ///     Data yang Disediakan:
    ///     - Net value harian broker pada saham tersebut
    ///     - Jumlah hari aktif trading
    ///     - Total akumulasi net value
    ///     - Rata-rata net value per hari aktif
    ///     - Hari dengan net value tertinggi (peak)
    /// 
    ///     Jenis Data: Time Series (daily)
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Melacak akumulasi broker pada saham tertentu
    ///     - Analisa pola trading broker
    ///     - Identifikasi hari-hari aktif broker
    ///     - Monitoring aktivitas bandar
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Track a specific broker's trading activity on a specific stock in daily calendar format.
    /// 
    ///     Data Provided:
    ///     - Daily net value of broker on the stock
    ///     - Number of active trading days
    ///     - Total accumulated net value
    ///     - Average net value per active day
    ///     - Day with highest net value (peak)
    /// 
    ///     Data Type: Time Series (daily)
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Track broker accumulation on a specific stock
    ///     - Analyze broker trading patterns
    ///     - Identify broker's active days
    ///     - Monitor market maker activity
    pub async fn broker_stalker(&self, broker: &str, stock: &str, from: &str, to: &str, investor: &str, market: &str) -> Result<BrokerStalkerResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/stalker/broker/{}/{}", broker, stock);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("investor", Some(investor.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar semua saham yang diperdagangkan broker tertentu dalam periode waktu tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Daftar saham dengan net value (dalam IDR atau lot)
    ///     - Jumlah transaksi per saham
    ///     - Buy dominance per saham (persentase)
    ///     - Summary statistik: total saham, total net, top saham, konsentrasi, buy dominance keseluruhan
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Melihat portfolio broker
    ///     - Analisa saham favorit broker
    ///     - Tracking akumulasi/distribusi broker
    ///     - Identifikasi pola trading broker
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get list of all stocks traded by a specific broker within a specific time period.
    /// 
    ///     Data Provided:
    ///     - Stock list with net value (in IDR or lots)
    ///     - Transaction count per stock
    ///     - Buy dominance per stock (percentage)
    ///     - Summary statistics: total stocks, total net, top stock, concentration, overall buy dominance
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - View broker portfolio
    ///     - Analyze broker's favorite stocks
    ///     - Track broker accumulation/distribution
    ///     - Identify broker trading patterns
    pub async fn broker_stalker_list(&self, code: &str, from: &str, to: &str, investor: &str, market: &str) -> Result<BrokerStalkerListResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/stalker/list/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("investor", Some(investor.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data infografik broker summary untuk saham tertentu dengan visualisasi yang menarik pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Infografik broker summary per saham
    ///     - Data visualisasi dengan warna dan label
    ///     - Breakdown buy/sell per investor type
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa broker activity
    ///     - Visualisasi broker activity
    ///     - Infografik broker summary
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get broker summary infographic data for a specific stock with attractive visualization for a specific period.
    /// 
    ///     Data Provided:
    ///     - Broker summary infographic per stock
    ///     - Visualization data with colors and labels
    ///     - Buy/sell breakdown per investor type
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Broker activity analysis
    ///     - Broker activity visualization
    ///     - Broker summary infographic
    pub async fn summary_stock_chart(&self, code: &str, from: &str, to: &str, scope: &str, market: &str) -> Result<Vec<SummaryStockChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/summary-chart/stock/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("scope", Some(scope.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data infografik broker summary untuk broker tertentu dengan visualisasi yang menarik pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Infografik broker summary per broker
    ///     - Data visualisasi dengan warna dan label
    ///     - Breakdown buy/sell per investor type
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa broker activity
    ///     - Visualisasi broker activity
    ///     - Infografik broker performance
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get broker summary infographic data for a specific broker with attractive visualization for a specific period.
    /// 
    ///     Data Provided:
    ///     - Broker summary infographic per broker
    ///     - Visualization data with colors and labels
    ///     - Buy/sell breakdown per investor type
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Cross Section / Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Broker activity analysis
    ///     - Broker activity visualization
    ///     - Broker performance infographic
    pub async fn summary_broker_chart(&self, code: &str, from: &str, to: &str, scope: &str, market: &str) -> Result<Vec<SummaryBrokerChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/summary-chart/broker/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("scope", Some(scope.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data inventory chart untuk saham tertentu dengan visualisasi broker activity pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi inventory broker per saham
    ///     - Data time series dengan broker activity
    ///     - Analisis akumulasi dan distribusi
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisis inventory broker
    ///     - Visualisasi akumulasi/distribusi
    ///     - Analisa broker activity
    ///     - Monitoring broker activity
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get inventory chart data for a specific stock with broker activity visualization for a specific period.
    /// 
    ///     Data Provided:
    ///     - Broker inventory visualization per stock
    ///     - Time series data with broker activity
    ///     - Accumulation and distribution analysis
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Broker inventory analysis
    ///     - Accumulation/distribution visualization
    ///     - Broker activity analysis
    ///     - Broker activity monitoring
    pub async fn inventory_stock_chart(&self, code: &str, from: &str, to: &str, scope: &str, investor: &str, limit: Option<&str>, market: &str, filter: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<InventoryStockChartResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/inventory-chart/stock/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("scope", Some(scope.to_string())),
            ("investor", Some(investor.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("market", Some(market.to_string())),
            ("filter", filter.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data inventory chart untuk broker tertentu dengan visualisasi saham activity pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi inventory broker per saham
    ///     - Data time series dengan saham activity
    ///     - Analisis akumulasi dan distribusi
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Parameter Filter (Optional):
    ///     - **filter_operator**: Operator perbandingan untuk filter scope (<, >, =, >=, <=, !=)
    ///     - **filter_value**: Nilai untuk perbandingan filter scope
    /// 
    ///     Filter otomatis menggunakan kolom dari parameter 'scope' yang dikombinasikan dengan 'investor'.
    ///     Format kolom filter: net_{investor}_{scope} (contoh: net_d_val, net_f_vol, net_all_freq)
    /// 
    ///     Contoh Filter:
    ///     - scope=val, investor=d, filter_operator=>, filter_value=1000000 → Net distributor value > 1 juta
    ///     - scope=vol, investor=f, filter_operator=>=, filter_value=1000 → Foreign volume >= 1000
    ///     - scope=freq, investor=all, filter_operator=>, filter_value=100 → All frequency > 100
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2016-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa inventory broker
    ///     - Visualisasi saham favorit broker
    ///     - Analisa broker activity
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get inventory chart data for a specific broker with stock activity visualization for a specific period.
    /// 
    ///     Data Provided:
    ///     - Broker inventory visualization per stock
    ///     - Time series data with stock activity
    ///     - Accumulation and distribution analysis
    ///     - Chart-ready data for visualization
    /// 
    ///     Filter Parameters (Optional):
    ///     - **filter_operator**: Comparison operator for scope filter (<, >, =, >=, <=, !=)
    ///     - **filter_value**: Value for scope filter comparison
    /// 
    ///     Filter automatically uses the column from 'scope' parameter combined with 'investor'.
    ///     Filter column format: net_{investor}_{scope} (example: net_d_val, net_f_vol, net_all_freq)
    /// 
    ///     Filter Examples:
    ///     - scope=val, investor=d, filter_operator=>, filter_value=1000000 → Net distributor value > 1 million
    ///     - scope=vol, investor=f, filter_operator=>=, filter_value=1000 → Foreign volume >= 1000
    ///     - scope=freq, investor=all, filter_operator=>, filter_value=100 → All frequency > 100
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2016-01-01
    /// 
    ///     Usage Examples:
    ///     - Broker inventory analysis
    ///     - Broker's favorite stocks visualization
    ///     - Broker activity analysis
    pub async fn inventory_broker_chart(&self, code: &str, from: &str, to: &str, scope: &str, investor: &str, limit: Option<&str>, market: &str, filter: Option<&str>, filter_operator: Option<&str>, filter_value: Option<&str>) -> Result<Vec<InventoryBrokerChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/inventory-chart/broker/{}", code);
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("scope", Some(scope.to_string())),
            ("investor", Some(investor.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("market", Some(market.to_string())),
            ("filter", filter.map(|v| v.to_string())),
            ("filter_operator", filter_operator.map(|v| v.to_string())),
            ("filter_value", filter_value.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data momentum chart untuk analisis transaksi real-time dengan detail HAKA HAKI pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi momentum trading real-time
    ///     - Data HAKA HAKI (Harga, Akumulasi, Kenaikan, Akumulasi, Kenaikan, Indikator)
    ///     - Analisis momentum per interval waktu
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Data Awal: 7 hari terakhir
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa momentum trading
    ///     - Identifikasi pola momentum
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get momentum chart data for real-time transaction analysis with HAKA HAKI details for a specific period.
    /// 
    ///     Data Provided:
    ///     - Real-time trading momentum visualization
    ///     - HAKA HAKI data (Price, Accumulation, Increase, Accumulation, Increase, Indicator)
    ///     - Momentum analysis per time interval
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Initial Data: 7 hari kebelakang
    /// 
    ///     Usage Examples:
    ///     - Trading momentum analysis
    ///     - Momentum pattern identification
    pub async fn momentum_chart(&self, code: &str, date: &str, range: &str, scope: &str) -> Result<Vec<MomentumChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/momentum-chart/{}", code);
        let query = [
            ("date", Some(date.to_string())),
            ("range", Some(range.to_string())),
            ("scope", Some(scope.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data intraday inventory chart untuk analisa transaksi dengan detail broker activity pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi inventory broker intraday
    ///     - Data time series dengan broker activity real-time
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 7 hari terakhir
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa inventory broker intraday
    ///     - Visualisasi akumulasi/distribusi real-time
    ///     - Analisa broker activity real-time
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get intraday inventory chart data for transaction analysis with broker activity details for a specific period.
    /// 
    ///     Data Provided:
    ///     - Intraday broker inventory visualization
    ///     - Time series data with real-time broker activity
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: Past 7 day
    /// 
    ///     Usage Examples:
    ///     - Intraday broker inventory analysis
    ///     - Real-time accumulation/distribution visualization
    ///     - Real-time broker activity analysis
    pub async fn intraday_inventory_chart(&self, code: &str, date: &str, range: &str, r#type: &str, total: &str, buyer: &str, seller: &str, filter: Option<&str>, market: &str) -> Result<IntradayInventoryChartResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/intraday-inventory-chart/{}", code);
        let query = [
            ("date", Some(date.to_string())),
            ("range", Some(range.to_string())),
            ("type", Some(r#type.to_string())),
            ("total", Some(total.to_string())),
            ("buyer", Some(buyer.to_string())),
            ("seller", Some(seller.to_string())),
            ("filter", filter.map(|v| v.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data sankey chart untuk visualisasi crossing transaksi dengan detail broker flow pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi crossing transaksi dengan sankey chart
    ///     - Data flow broker dengan visualisasi yang menarik
    ///     - Analisa crossing pattern
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 7 hari kebelakang
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa crossing transaksi
    ///     - Visualisasi broker flow
    ///     - Analisa broker flow
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get sankey chart data for crossing transaction visualization with broker flow details for a specific period.
    /// 
    ///     Data Provided:
    ///     - Crossing transaction visualization with sankey chart
    ///     - Broker flow data with attractive visualization
    ///     - Crossing pattern analysis
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: Past 7 day
    /// 
    ///     Usage Examples:
    ///     - Crossing transaction analysis
    ///     - Broker flow visualization
    ///     - Broker flow analysis
    pub async fn sankey_chart(&self, code: &str, date: &str, r#type: &str, buyer: &str, seller: &str, market: &str) -> Result<SankeyChartResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/sankey-chart/{}", code);
        let query = [
            ("date", Some(date.to_string())),
            ("type", Some(r#type.to_string())),
            ("buyer", Some(buyer.to_string())),
            ("seller", Some(seller.to_string())),
            ("market", Some(market.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan tabel transaksi harga perdagangan harian saham dengan detail volume dan frekuensi pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Tabel harga dengan volume buy/sell
    ///     - Frekuensi transaksi per harga
    ///     - Data real-time perdagangan
    ///     - Detail transaksi harian
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Data Awal: 7 hari kebelakang
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa level harga support/resistance
    ///     - Identifikasi area konsentrasi volume
    ///     - Analisa market depth
    ///     - Strategi trading development
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get daily stock trading price transaction table with volume and frequency details for a specific period.
    /// 
    ///     Data Provided:
    ///     - Price table with buy/sell volume
    ///     - Transaction frequency per price
    ///     - Real-time trading data
    ///     - Daily transaction details
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Initial Data: Past 7 day
    /// 
    ///     Usage Examples:
    ///     - Support/resistance price level analysis
    ///     - Volume concentration area identification
    ///     - Market depth analysis
    ///     - Trading strategy development
    pub async fn price_table(&self, code: &str, date: &str) -> Result<Vec<PriceTableResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/price-table/{}", code);
        let query = [
            ("date", Some(date.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan tabel transaksi waktu perdagangan harian saham dengan detail OHLCV per interval waktu pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Tabel waktu dengan data OHLCV
    ///     - Volume dan nilai transaksi per interval
    ///     - Data buy/sell per interval waktu
    ///     - Data real-time perdagangan
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Data Awal: 7 hari kebelakang
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa intraday trading pattern
    ///     - Identifikasi waktu trading aktif
    ///     - Analisa volume per waktu
    ///     - Strategi trading development
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get daily stock trading time transaction table with OHLCV details per time interval for a specific period.
    /// 
    ///     Data Provided:
    ///     - Time table with OHLCV data
    ///     - Volume and transaction value per interval
    ///     - Buy/sell data per time interval
    ///     - Real-time trading data
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time
    /// 
    ///     Initial Data: Past 7 day
    /// 
    ///     Usage Examples:
    ///     - Intraday trading pattern analysis
    ///     - Active trading time identification
    ///     - Volume per time analysis
    ///     - Trading strategy development
    pub async fn time_table(&self, code: &str, date: &str, range: &str) -> Result<Vec<TimeTableResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/time-table/{}", code);
        let query = [
            ("date", Some(date.to_string())),
            ("range", Some(range.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan tabel perubahan harga saham harian dengan data historis lengkap pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Data harga harian historis
    ///     - Volume dan nilai transaksi harian
    ///     - Persentase perubahan harga
    ///     - Data time series lengkap
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa performa saham
    ///     - Tracking perubahan harga harian
    ///     - Analisa trend harga
    ///     - Performance benchmarking
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get daily stock price change table with complete historical data for a specific period.
    /// 
    ///     Data Provided:
    ///     - Historical daily price data
    ///     - Daily volume and transaction value
    ///     - Price change percentage
    ///     - Complete time series data
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Usage Examples:
    ///     - Stock performance analysis
    ///     - Daily price change tracking
    ///     - Price trend analysis
    ///     - Performance benchmarking
    pub async fn price_diary(&self, code: &str) -> Result<Vec<PriceDiaryResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/price-diary/{}", code);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan tabel perubahan harga saham bulanan dengan analisis seasonality pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Data harga bulanan historis
    ///     - Volume dan nilai transaksi bulanan
    ///     - Persentase perubahan harga bulanan
    ///     - Analisa seasonality pattern
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: End of Day (EOD) setiap hari jam 18:00 WIB
    /// 
    ///     Data Awal: 2010-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa seasonality saham
    ///     - Tracking performa bulanan
    ///     - Analisa trend jangka panjang
    ///     - Performance benchmarking
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get monthly stock price change table with seasonality analysis for a specific period.
    /// 
    ///     Data Provided:
    ///     - Historical monthly price data
    ///     - Monthly volume and transaction value
    ///     - Monthly price change percentage
    ///     - Seasonality pattern analysis
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: End of Day (EOD) every day at 18:00 WIB
    /// 
    ///     Initial Data: 2010-01-01
    /// 
    ///     Usage Examples:
    ///     - Stock seasonality analysis
    ///     - Monthly performance tracking
    ///     - Long-term trend analysis
    ///     - Performance benchmarking
    pub async fn price_seasonality(&self, code: &str, range: &str) -> Result<Vec<PriceSeasonalityResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/price-seasonality/{}", code);
        let query = [
            ("range", Some(range.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar perubahan kepemilikan saham diatas 5% berdasarkan laporan bursa pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Perubahan kepemilikan saham diatas 5%
    ///     - Data transaksi insider
    ///     - Informasi pemegang saham mayoritas
    ///     - Data time series perubahan kepemilikan
    ///     - Pagination untuk data besar
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa insider trading
    ///     - Tracking perubahan kepemilikan mayoritas
    ///     - Due diligence investasi
    ///     - Corporate governance analysis
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of ownership changes above 5% based on exchange reports for a specific period.
    /// 
    ///     Data Provided:
    ///     - Ownership changes above 5%
    ///     - Insider transaction data
    ///     - Majority shareholder information
    ///     - Time series data of ownership changes
    ///     - Pagination for large data
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Insider trading analysis
    ///     - Majority ownership change tracking
    ///     - Investment due diligence
    ///     - Corporate governance analysis
    pub async fn shareholder_above(&self, code: Option<&str>, broker: Option<&str>, name: Option<&str>, from: &str, to: &str, page: Option<&str>, limit: Option<&str>) -> Result<ShareholderAboveResponse, crate::error::InvezgoError> {
        let path = "/analysis/shareholder-above".to_string();
        let query = [
            ("code", code.map(|v| v.to_string())),
            ("broker", broker.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("page", page.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data chart untuk visualisasi kepemilikan saham diatas 5% pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi chart kepemilikan saham diatas 5%
    ///     - Data time series perubahan kepemilikan
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa trend kepemilikan
    ///     - Visualisasi perubahan kepemilikan diatas 5%
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get chart data for ownership visualization above 5% for a specific period.
    /// 
    ///     Data Provided:
    ///     - Ownership chart visualization above 5%
    ///     - Time series data of ownership changes
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Ownership trend analysis
    ///     - Ownership change visualization above 5%
    pub async fn shareholder_above_chart(&self, code: &str, broker: Option<&str>, name: Option<&str>, date: Option<&str>) -> Result<Vec<ShareholderAboveChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder-above-chart/{}", code);
        let query = [
            ("broker", broker.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("date", date.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar perubahan kepemilikan saham diatas 1% berdasarkan laporan bursa pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Perubahan kepemilikan saham diatas 1%
    ///     - Data transaksi insider
    ///     - Informasi pemegang saham mayoritas
    ///     - Data time series perubahan kepemilikan
    ///     - Pagination untuk data besar
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa insider trading
    ///     - Tracking perubahan kepemilikan mayoritas
    ///     - Due diligence investasi
    ///     - Corporate governance analysis
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of ownership changes above 1% based on exchange reports for a specific period.
    /// 
    ///     Data Provided:
    ///     - Ownership changes above 1%
    ///     - Insider transaction data
    ///     - Majority shareholder information
    ///     - Time series data of ownership changes
    ///     - Pagination for large data
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Insider trading analysis
    ///     - Majority ownership change tracking
    ///     - Investment due diligence
    ///     - Corporate governance analysis
    pub async fn shareholder_one(&self, code: Option<&str>, name: Option<&str>, from: &str, to: &str, page: Option<&str>, limit: Option<&str>) -> Result<ShareholderOneResponse, crate::error::InvezgoError> {
        let path = "/analysis/shareholder-one".to_string();
        let query = [
            ("code", code.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("page", page.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data chart untuk visualisasi kepemilikan saham diatas 1% pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi chart kepemilikan saham diatas 1%
    ///     - Data time series perubahan kepemilikan
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2026-02-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa trend kepemilikan
    ///     - Visualisasi perubahan kepemilikan diatas 1%
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get chart data for ownership visualization above 1% for a specific period.
    /// 
    ///     Data Provided:
    ///     - Ownership chart visualization above 5%
    ///     - Time series data of ownership changes
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2026-02-01
    /// 
    ///     Usage Examples:
    ///     - Ownership trend analysis
    ///     - Ownership change visualization above 1%
    pub async fn shareholder_one_chart(&self, code: &str, broker: Option<&str>, name: Option<&str>, date: Option<&str>) -> Result<Vec<ShareholderOneChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/shareholder-one-chart/{}", code);
        let query = [
            ("broker", broker.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("date", date.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan daftar perubahan kepemilikan saham insider (direksi, komisaris, pemegang saham mayoritas) pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Perubahan kepemilikan saham insider
    ///     - Data transaksi direksi dan komisaris
    ///     - Informasi pemegang saham mayoritas
    ///     - Data time series perubahan kepemilikan
    ///     - Pagination untuk data besar
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisis insider trading
    ///     - Tracking aktivitas direksi/komisaris
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get a list of insider ownership changes (directors, commissioners, majority shareholders) for a specific period.
    /// 
    ///     Data Provided:
    ///     - Insider ownership changes
    ///     - Directors and commissioners transaction data
    ///     - Majority shareholder information
    ///     - Time series data of ownership changes
    ///     - Pagination for large data
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Insider trading analysis
    ///     - Directors/commissioners activity tracking
    pub async fn insider(&self, code: Option<&str>, name: Option<&str>, from: &str, to: &str, page: Option<&str>, limit: Option<&str>) -> Result<InsiderResponse, crate::error::InvezgoError> {
        let path = "/analysis/shareholder-insider".to_string();
        let query = [
            ("code", code.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("page", page.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data chart untuk visualisasi insider trading pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi chart insider trading
    ///     - Data time series perubahan kepemilikan insider
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisis trend insider trading
    ///     - Visualisasi aktivitas insider
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get chart data for insider trading visualization for a specific period.
    /// 
    ///     Data Provided:
    ///     - Insider trading chart visualization
    ///     - Time series data of insider ownership changes
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Insider trading trend analysis
    ///     - Insider activity visualization
    pub async fn insider_chart(&self, code: &str, name: Option<&str>, date: Option<&str>) -> Result<Vec<InsiderChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/insider-chart/{}", code);
        let query = [
            ("name", name.map(|v| v.to_string())),
            ("date", date.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data laporan keuangan perusahaan berdasarkan jenis laporan dan periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Laporan Neraca (Balance Sheet)
    ///     - Laporan Laba Rugi (Income Statement)
    ///     - Laporan Arus Kas (Cash Flow)
    ///     - Data historis laporan keuangan
    ///     - Struktur data hierarkis dengan parent-child relationship
    /// 
    ///     Jenis Data: Panel (data laporan keuangan)
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa fundamental perusahaan
    ///     - Due diligence investasi
    ///     - Analisa rasio keuangan
    ///     - Tracking kinerja keuangan
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get company financial statement data based on report type and specific period.
    /// 
    ///     Data Provided:
    ///     - Balance Sheet
    ///     - Income Statement
    ///     - Cash Flow Statement
    ///     - Historical financial statement data
    ///     - Hierarchical data structure with parent-child relationship
    /// 
    ///     Data Type: Panel (financial statement data)
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Company fundamental analysis
    ///     - Investment due diligence
    ///     - Financial ratio analysis
    ///     - Financial performance tracking
    pub async fn financial_statement(&self, code: &str, statement: &str, r#type: &str, limit: Option<&str>) -> Result<FinancialStatementResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/financial-statement/{}", code);
        let query = [
            ("statement", Some(statement.to_string())),
            ("type", Some(r#type.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data chart untuk visualisasi laporan keuangan pada periode tertentu.
    /// 
    ///       Data yang Disediakan:
    ///       - Visualisasi chart laporan keuangan
    ///       - Data time series laporan keuangan
    ///       - Data chart-ready untuk visualisasi
    /// 
    ///       Jenis Data: Time Series
    /// 
    ///       Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///       Data Awal: 2020-01-01
    /// 
    ///       Contoh Penggunaan:
    ///       - Analisa trend keuangan
    ///       - Visualisasi laporan keuangan per akun
    /// 
    ///       ---
    ///       [English]
    /// 
    ///       Get chart data for financial statement visualization for a specific period.
    /// 
    ///       Data Provided:
    ///       - Financial statement chart visualization
    ///       - Time series financial statement data
    ///       - Chart-ready data for visualization
    /// 
    ///       Data Type: Time Series
    /// 
    ///       Update: Real-time according to each company's report
    /// 
    ///       Initial Data: 2020-01-01
    /// 
    ///       Usage Examples:
    ///       - Financial trend analysis
    ///       - Financial statement visualization per account
    pub async fn financial_statement_chart(&self, code: &str, statement: &str, r#type: &str, limit: Option<&str>, account: &str) -> Result<Vec<FinancialStatementChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/financial-statement-chart/{}", code);
        let query = [
            ("statement", Some(statement.to_string())),
            ("type", Some(r#type.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("account", Some(account.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data key statistics dan metrik keuangan penting perusahaan pada periode tertentu.
    /// 
    ///       Data yang Disediakan:
    ///       - Rasio keuangan penting
    ///       - Metrik kinerja perusahaan
    ///       - Data historis key statistics
    ///       - Indikator fundamental
    ///       - Struktur data hierarkis
    /// 
    ///       Jenis Data: Panel (data metrik keuangan)
    /// 
    ///       Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///       Data Awal: 2020-01-01
    /// 
    ///       Contoh Penggunaan:
    ///       - Analisa rasio keuangan
    ///       - Benchmarking perusahaan
    ///       - Analisa fundamental
    ///       - Due diligence investasi
    /// 
    ///       ---
    ///       [English]
    /// 
    ///       Get key statistics and important financial metrics of a company for a specific period.
    /// 
    ///       Data Provided:
    ///       - Important financial ratios
    ///       - Company performance metrics
    ///       - Historical key statistics data
    ///       - Fundamental indicators
    ///       - Hierarchical data structure
    /// 
    ///       Data Type: Panel (financial metrics data)
    /// 
    ///       Update: Real-time according to each company's report
    /// 
    ///       Initial Data: 2020-01-01
    /// 
    ///       Usage Examples:
    ///       - Financial ratio analysis
    ///       - Company benchmarking
    ///       - Fundamental analysis
    ///       - Investment due diligence
    pub async fn keystat(&self, code: &str, r#type: &str, limit: Option<&str>) -> Result<KeystatResponse, crate::error::InvezgoError> {
        let path = format!("/analysis/keystat/{}", code);
        let query = [
            ("type", Some(r#type.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan data chart untuk visualisasi key statistics keuangan pada periode tertentu.
    /// 
    ///     Data yang Disediakan:
    ///     - Visualisasi chart key statistics
    ///     - Data time series key statistics
    ///     - Data chart-ready untuk visualisasi
    /// 
    ///     Jenis Data: Time Series
    /// 
    ///     Update: Real-time sesuai laporan masing-masing perusahaan
    /// 
    ///     Data Awal: 2020-01-01
    /// 
    ///     Contoh Penggunaan:
    ///     - Analisa trend key statistics
    ///     - Visualisasi metrik keuangan per akun
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get chart data for key statistics visualization for a specific period.
    /// 
    ///     Data Provided:
    ///     - Key statistics chart visualization
    ///     - Time series key statistics data
    ///     - Chart-ready data for visualization
    /// 
    ///     Data Type: Time Series
    /// 
    ///     Update: Real-time according to each company's report
    /// 
    ///     Initial Data: 2020-01-01
    /// 
    ///     Usage Examples:
    ///     - Key statistics trend analysis
    ///     - Financial metrics visualization per account
    pub async fn keystat_chart(&self, code: &str, r#type: &str, limit: Option<&str>, name: &str) -> Result<Vec<KeystatChartResponse>, crate::error::InvezgoError> {
        let path = format!("/analysis/keystat-chart/{}", code);
        let query = [
            ("type", Some(r#type.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("name", Some(name.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan jadwal corporate action berdasarkan filter tertentu.
    /// 
    ///     Filter yang tersedia (Wajib pilih salah satu):
    ///     - Code: Kode saham tertentu
    ///     - Date: Tanggal tertentu
    ///     - Type: Jenis corporate action
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get corporate action schedule based on specific filters.
    /// 
    ///     Available Filters (Must choose at least one):
    ///     - Code: Specific stock code
    ///     - Date: Specific date
    ///     - Type: Corporate action type
    pub async fn get_calendar(&self, code: Option<&str>, r#type: Option<&str>, page: Option<&str>, limit: Option<&str>) -> Result<GetCalendarResponse, crate::error::InvezgoError> {
        let path = "/analysis/calendar".to_string();
        let query = [
            ("code", code.map(|v| v.to_string())),
            ("type", r#type.map(|v| v.to_string())),
            ("page", page.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Mendapatkan antrian order (order tracking) untuk saham pada harga, sisi, halaman, dan limit tertentu.
    /// 
    ///     Fitur:
    ///     - Realtime order queue tracking dari third-party API.
    ///     - Menampilkan detail antrian order: waktu order, ID order, volume order, volume yang masih open (outstanding), volume yang sudah done (terisi), nilai order (order_value), nilai yang masih open (open_value), dan nilai yang sudah done (done_value).
    ///     - Dukungan fallback ke penyedia data alternatif saat terjadi gangguan.
    ///     - Caching otomatis di luar jam perdagangan bursa.
    /// 
    ///     Jenis Data: Real-time
    /// 
    ///     Update: Real-time selama jam perdagangan
    /// 
    ///     Contoh Penggunaan:
    ///     - Melacak antrian antrean order pada level harga tertentu
    ///     - Analisis distribusi antrean bid/offer secara detail
    ///     - Mengidentifikasi order besar (institutional order) dalam antrean
    /// 
    ///     ---
    ///     [English]
    /// 
    ///     Get order queue (order tracking) for a stock at a specific price, side, page, and limit.
    /// 
    ///     Features:
    ///     - Real-time order queue tracking from third-party APIs.
    ///     - Displays detailed order queue: order time, order ID, order volume, open volume (outstanding), completed volume (done), order value (order_value), open value (open_value), and completed value (done_value).
    ///     - Alternative data provider fallback support during outages.
    ///     - Automatic caching outside trading hours.
    /// 
    ///     Data Type: Real-time
    /// 
    ///     Update: Real-time during trading hours
    /// 
    ///     Usage Examples:
    ///     - Track order queue at a specific price level
    ///     - Detailed bid/offer queue distribution analysis
    ///     - Identify large orders (institutional orders) in the queue
    pub async fn get_order_queue(&self, code: &str, price: &str, side: &str, page: Option<&str>, limit: Option<&str>) -> Result<Vec<OrderQueueResponseItem>, crate::error::InvezgoError> {
        let path = format!("/analysis/queue/{}", code);
        let query = [
            ("price", Some(price.to_string())),
            ("side", Some(side.to_string())),
            ("page", page.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }

}
