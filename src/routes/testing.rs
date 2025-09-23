use leptos::*;
use leptos_qr_scanner::QrScanner;

#[component]
pub fn QrScannerView() -> impl IntoView {
    view! {
        <div>
            <h2>"Scan QR Code"</h2>
            <QrScanner
                on_code=move |code| {
                    log!("Scanned QR code: {}", code);
                    // You can call a backend API or update state here
                }
            />
        </div>
    }
}
