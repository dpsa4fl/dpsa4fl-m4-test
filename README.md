
# Test application for dp noise (dpsa milestone 4)

This repo contains code to test that adding noise to submitted gradient shares with
our [janus fork](https://github.com/dpsa-project/janus/tree/dpsa-m4-release) works.

## Run
In order to run the test, first make sure that the newest janus server fork is running.
For this, follow the instruction [here](https://github.com/dpsa-project/dpsa4fl-testing-infrastructure).

Run the executable with `cargo run`.

## Result
You should see a result similar to the following. Please note that submitting the 60000 element
vector could take a while. Also note that we print only the first 15 elements of that vector.

```
Submitting gradient with 3 elements.
Creating controller
Starting round for session id 29197.
started round with task id pSKxw4gOL7z6a53UPnXPBlwEMW3YD71TR2xoqJ1DEiw
submitting gradient 1
submitting vector: [0.0625, 0.0625, 0.0625]
submitting gradient 2
submitting vector: [0.0625, 0.0625, 0.0625]
collecting
patched host and port are: 127.0.0.1 -:- 9991
collecting result now
got result, it is:
[0.12499527586624026, 0.1250085891224444, 0.12499268259853125]


Submitting gradient with 60000 elements.
Creating controller
Starting round for session id 30027.
started round with task id QdyR3Olde1msI6pLZkPZfuGcN2O40tMfaJv0xFTwpgM
submitting gradient 1
submitting vector: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
submitting gradient 2
submitting vector: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
collecting
patched host and port are: 127.0.0.1 -:- 9991
collecting result now
got result, it is:
[1.1201482266187668e-5, 4.591420292854309e-6, -5.480833351612091e-6, -4.2943283915519714e-6, -9.527429938316345e-7, 3.855675458908081e-7, -1.1137686669826508e-5, -5.323905497789383e-6, -7.22147524356842e-6, 1.5208497643470764e-6, 4.866160452365875e-6, -3.6945566534996033e-6, -7.731840014457703e-6, -2.437736839056015e-6, 3.952067345380783e-6]
```
