package org.cochranblock.whyyoulying;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebView;
import android.webkit.WebSettings;
import android.widget.LinearLayout;
import android.widget.Button;
import android.view.ViewGroup;

/**
 * Main activity: WebView rendering the fraud detection demo report.
 * Rust detection engine called via JNI. HTML report displayed in WebView.
 */
public class MainActivity extends Activity {

    static {
        try {
            System.loadLibrary("whyyoulying_android");
        } catch (UnsatisfiedLinkError e) {
            // JNI lib not available — fall back to baked HTML
        }
    }

    /** JNI: run demo detection, return HTML report string. */
    private static native String runDemoHtml();

    /** JNI: run demo detection, return JSON string. */
    private static native String runDemoJson();

    /** JNI: get SPDX SBOM string. */
    private static native String getSbom();

    private WebView webView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);

        // Toolbar
        LinearLayout toolbar = new LinearLayout(this);
        toolbar.setBackgroundColor(0xFF003366);
        toolbar.setPadding(16, 12, 16, 12);

        Button btnDemo = makeButton("Demo Report");
        Button btnSbom = makeButton("SBOM");
        Button btnJson = makeButton("JSON");
        toolbar.addView(btnDemo);
        toolbar.addView(btnSbom);
        toolbar.addView(btnJson);
        root.addView(toolbar);

        // WebView
        webView = new WebView(this);
        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(false);
        settings.setBuiltInZoomControls(true);
        settings.setDisplayZoomControls(false);
        webView.setLayoutParams(new LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.MATCH_PARENT));
        root.addView(webView);

        setContentView(root);

        // Button handlers
        btnDemo.setOnClickListener(v -> loadDemoReport());
        btnSbom.setOnClickListener(v -> loadSbom());
        btnJson.setOnClickListener(v -> loadJson());

        // Load demo on start
        loadDemoReport();
    }

    private void loadDemoReport() {
        String html;
        try {
            html = runDemoHtml();
        } catch (UnsatisfiedLinkError e) {
            html = fallbackHtml("JNI library not loaded. Build with cargo-ndk for Android targets.");
        }
        webView.loadDataWithBaseURL(null, html, "text/html", "UTF-8", null);
    }

    private void loadSbom() {
        String sbom;
        try {
            sbom = getSbom();
        } catch (UnsatisfiedLinkError e) {
            sbom = "JNI library not loaded.";
        }
        String html = "<html><body><pre style='font-size:12px;padding:16px'>" +
            sbom.replace("<", "&lt;") + "</pre></body></html>";
        webView.loadDataWithBaseURL(null, html, "text/html", "UTF-8", null);
    }

    private void loadJson() {
        String json;
        try {
            json = runDemoJson();
        } catch (UnsatisfiedLinkError e) {
            json = "JNI library not loaded.";
        }
        String html = "<html><body><pre style='font-size:11px;padding:16px'>" +
            json.replace("<", "&lt;") + "</pre></body></html>";
        webView.loadDataWithBaseURL(null, html, "text/html", "UTF-8", null);
    }

    private String fallbackHtml(String msg) {
        return "<html><body style='font-family:sans-serif;padding:20px'>" +
            "<h2 style='color:#003366'>whyyoulying</h2>" +
            "<p>" + msg + "</p>" +
            "<p>Build: <code>cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release</code></p>" +
            "</body></html>";
    }

    private Button makeButton(String label) {
        Button b = new Button(this);
        b.setText(label);
        b.setTextColor(0xFFFFFFFF);
        b.setBackgroundColor(0x00000000);
        b.setPadding(24, 8, 24, 8);
        LinearLayout.LayoutParams lp = new LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.WRAP_CONTENT,
            ViewGroup.LayoutParams.WRAP_CONTENT);
        lp.setMarginEnd(8);
        b.setLayoutParams(lp);
        return b;
    }
}
