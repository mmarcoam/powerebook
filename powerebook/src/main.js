import { open } from '@tauri-apps/plugin-dialog';
const { invoke } = window.__TAURI__.core;

document.addEventListener('DOMContentLoaded', () => {
    const scanBtn = document.getElementById('scan-btn');
    const backBtn = document.getElementById('back-btn');
    const libraryView = document.getElementById('library-view');
    const readerView = document.getElementById('reader-view');
    const library = document.getElementById('library');
    const readerTitle = document.getElementById('reader-title');

    function showLibrary() {
        libraryView.classList.remove('hidden');
        readerView.classList.add('hidden');
        document.getElementById('header-controls').classList.remove('hidden');
        backBtn.classList.add('hidden');
    }

    function showReader(title, format, filePath) {
        libraryView.classList.add('hidden');
        readerView.classList.remove('hidden');
        document.getElementById('header-controls').classList.add('hidden');
        backBtn.classList.remove('hidden');
        readerTitle.textContent = title;
        
        const area = document.getElementById('epub-area');
        area.innerHTML = `<p style="color: #4CAF50; text-align: center; padding-top: 250px; font-size: 1.2rem;">🚀 Abrindo "${title}" no SumatraPDF...</p>`;

        invoke('abrir_com_sumatra', { filePath: filePath })
            .then(() => setTimeout(() => showLibrary(), 2000))
            .catch((error) => {
                area.innerHTML = `<p style="color: #ff6b6b; text-align: center; padding: 50px;">Erro ao abrir:<br>${error}</p>`;
            });
    }

    function renderBooks(files) {
        library.innerHTML = ''; 
        if (files.length === 0) {
            library.innerHTML = '<p style="color: #aaa; grid-column: 1/-1; text-align: center;">Sua biblioteca está vazia. Clique em "Importar Livros" para adicionar.</p>';
            return;
        }

        files.forEach(filePath => {
            const fileName = filePath.split('\\').pop().split('/').pop();
            const ext = fileName.split('.').pop().toUpperCase();
            const title = fileName.replace(/\.[^/.]+$/, ""); 
            
            const card = document.createElement('div');
            card.className = 'book-card';
            // Adicionamos o botão de lixeira no HTML do cartão
            card.innerHTML = `
                <button class="delete-btn" title="Deletar este livro">🗑️</button>
                <div class="book-cover"></div>
                <div class="book-info">
                    <h3>${title}</h3>
                    <p>Clique para ler</p>
                    <span class="book-format">${ext}</span>
                </div>
            `;
            
            // Ao clicar no cartão (fora da lixeira), abre o livro
            card.addEventListener('click', (e) => {
                if (!e.target.classList.contains('delete-btn')) {
                    showReader(title, ext, filePath);
                }
            });

            // Lógica do botão de deletar
            const deleteBtn = card.querySelector('.delete-btn');
            deleteBtn.addEventListener('click', async (e) => {
                e.stopPropagation(); // Impede que o clique abra o livro
                
                if (confirm(`Tem certeza que deseja deletar "${title}"?\nEsta ação não pode ser desfeita.`)) {
                    try {
                        await invoke('delete_book', { filePath: filePath });
                        card.remove(); // Remove o cartão da tela instantaneamente
                        
                        // Se a biblioteca ficar vazia, mostra a mensagem
                        if (library.children.length === 0) {
                            library.innerHTML = '<p style="color: #aaa; grid-column: 1/-1; text-align: center;">Sua biblioteca está vazia. Clique em "Importar Livros" para adicionar.</p>';
                        }
                    } catch (error) {
                        alert("Erro ao deletar: " + error);
                    }
                }
            });

            library.appendChild(card);
        });
    }

    async function loadLibrary() {
        try {
            const files = await invoke('get_local_library');
            renderBooks(files);
        } catch (error) {
            console.error("Erro ao carregar biblioteca:", error);
        }
    }

    if (scanBtn) {
        scanBtn.addEventListener('click', async () => {
            try {
                const selectedPath = await open({
                    directory: true, multiple: false, title: "Selecione a pasta de origem para IMPORTAR os livros"
                });
                if (!selectedPath) return;
                const message = await invoke('import_books', { sourcePath: selectedPath });
                alert(message);
                loadLibrary();
            } catch (error) {
                alert("Erro ao importar: " + error);
            }
        });
    }

    if (backBtn) { backBtn.addEventListener('click', showLibrary); }

    loadLibrary();
});