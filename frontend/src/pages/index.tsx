import React, { useState } from 'react';
import axios from 'axios';


const Checkout : React.FC = () => {
    const [loading,setLoading] = useState(false);

    const handleCheckout = async() => {
        setLoading(true);
        try{
            const response = await axios.post(
                'http://localhost:4000/checkout',
            );
            if(response.status === 200){
                const {url} = response.data as {url: string};

                window.open(url,'_blank');
            } else {
                console.log('Unexpected response :', response);
            }
        } catch (error) {
            console.log('Error during checkout :', error);
        } finally {
            setLoading(false);
        }
    }

    return (
        <div style={styles.container}>
            <div style={styles.card}>
                <button onClick={handleCheckout} style={styles.button} disabled={loading}>
                    {loading ? "Processing..." : "Checkout"}
                </button>
            </div>
        </div>
    )
}

const styles = {
    container: {
        fontFamily: 'Arial, sans-serif',
        backgroundColor: '#e0f7fa',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        margin: 0,
        padding: '20px',
        boxSizing: 'border-box' as 'border-box',
    },
    card: {
        backgroundColor: '#ffffff',
        padding: '40px',
        borderRadius: '16px',
        boxShadow: '0 10px 20px rgba(0, 0, 0, 0.1)',
        textAlign: 'center' as 'center',
        width: '100%',
        maxWidth: '400px',
    },
    button: {
        backgroundColor: '#ff5722',
        color: 'white',
        border: 'none',
        padding: '15px 30px',
        borderRadius: '8px',
        cursor: 'pointer',
        fontSize: '18px',
        fontWeight: 'bold' as 'bold',
        transition: 'background-color 0.3s ease',
    },
};

export default Checkout;