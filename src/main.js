document.getElementById('inference').addEventListener('click', async () => {
    if (window.__TAURI__) {
      try {
        const result = await window.__TAURI__.invoke('run_inference', { modelPath: 'model.onnx', inputData: [1.0, 2.0, 3.0] });
        console.log('Inference result:', result);
      } catch (error) {
        console.error('Error:', error);
      }
    } else {
      console.log('Tauri API not available. Running in web debug mode.');
    }
  });
  