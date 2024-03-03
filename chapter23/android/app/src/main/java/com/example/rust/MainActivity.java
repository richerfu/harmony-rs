package com.example.rust;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

import com.example.rust.databinding.ActivityMainBinding;

public class MainActivity extends AppCompatActivity {

    // Used to load the 'rust' library on application startup.
    static {
        System.loadLibrary("ars");
    }

    private ActivityMainBinding binding;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        // Example of a call to a native method
        TextView tv = binding.sampleText;
        tv.setText(HelloRust());
    }

    /**
     * A native method that is implemented by the 'rust' native library,
     * which is packaged with this application.
     */
//    public native String stringFromJNI();

    public native String HelloRust();
}