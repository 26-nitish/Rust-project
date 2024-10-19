document.getElementById('decodeForm').addEventListener('submit', async function(e) {
    e.preventDefault();

    const formData = new FormData();
    const imageFile = document.getElementById('image').files[0];
    const password = document.getElementById('password').value;

    if (!imageFile) {
        alert('Please select a stego image.');
        return;
    }

    formData.append('image', imageFile);
    if (password) {
        formData.append('password', password);
    }

    try {
        const response = await fetch('http://localhost:8080/decode', {
            method: 'POST',
            body: formData
        });

        if (response.ok) {
            const message = await response.text();
            document.getElementById('secretMessage').innerText = message;
            document.getElementById('result').classList.remove('hidden');
        } else {
            const errorText = await response.text();
            alert(`Failed to decode message: ${errorText}`);
        }
    } catch (error) {
        console.error('Error:', error);
        alert('An error occurred while decoding the message.');
    }
});
