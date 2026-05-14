## Experiment 1.2

![Experiment 1.2](./screenshots/img_1.png)
Teks "hey hey" tercetak sebelum "done!". Hal ini terjadi karena proses spawner.spawn mengutus future secara asinkron tanpa memblokir proses thread utama. Eksekusi "hey hey" berjalan seketika, sementara "done!" harus menunggu TimerFuture menahan state selama 2 detik sebelum akhirnya executor melanjutkan eksekusinya.

## Experiment 1.3

![Experiment 1.3](./screenshots/1.3E.png)
Ketika beberapa task di-spawn, semuanya berjalan secara konkuren. "howdy", "howdy2", "howdy3" muncul nyaris bersamaan, diikuti jeda 2 detik, lalu "done", "done2", "done3" juga dieksekusi bersamaan. Penghapusan drop(spawner) menyebabkan program mengalami infinite loop (menggantung). drop(spawner) berfungsi memberi sinyal penutupan pada channel sender. Tanpanya, executor.run() akan terus berada dalam status listening karena mengira spawner masih aktif dan akan mengirimkan task baru.