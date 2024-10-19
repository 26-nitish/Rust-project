document.getElementById('encodeForm').addEventListener('submit', async function(e) {
    e.preventDefault();

    const formData = new FormData();
    const imageFile = document.getElementById('image').files[0];
    const message = document.getElementById('message').value;
    const password = document.getElementById('password').value;

    if (!imageFile || !message) {
        alert('Please provide both an image and a message.');
        return;
    }

    formData.append('image', imageFile);
    formData.append('message', message);
    if (password) {
        formData.append('password', password);
    }

    try {
        const response = await fetch('http://localhost:8080/encode', {
            method: 'POST',
            body: formData
        });

        if (response.ok) {
            const blob = await response.blob();
            const url = window.URL.createObjectURL(blob);

            const downloadLink = document.getElementById('downloadLink');
            downloadLink.href = url;
            downloadLink.download = 'stego_image.png';

            document.getElementById('result').classList.remove('hidden');
        } else {
            const errorText = await response.text();
            alert(`Failed to encode message: ${errorText}`);
        }
    } catch (error) {
        console.error('Error:', error);
        alert('An error occurred while encoding the message.');
    }
});
