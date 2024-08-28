
const Success : React.FC = () => {
    return (
        <div style={styles.container}>
            <div style={styles.card}>
                <h1 style={styles.message}>Payment Successful!</h1>
            </div>
        </div>
    )
}

const styles = {
    container: {
        fontFamily: 'Arial, sans-serif',
        backgroundColor: '#f0f0f0',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        margin: 0,
    },
    card: {
        backgroundColor: '#ffffff',
        padding: '20px',
        borderRadius: '8px',
        boxShadow: '0 4px 8px rgba(0, 0, 0, 0.1)',
        textAlign: 'center',
        width: '200px',
    },
    message: {
        color: '#28a745',
        margin: 0,
    },
};

export default Success;