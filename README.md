## Experiment 1.2

![img_1.png](img_1.png)
Teks "hey hey" tercetak sebelum "done!". Hal ini terjadi karena proses spawner.spawn mengutus future secara asinkron tanpa memblokir proses thread utama. Eksekusi "hey hey" berjalan seketika, sementara "done!" harus menunggu TimerFuture menahan state selama 2 detik sebelum akhirnya executor melanjutkan eksekusinya.