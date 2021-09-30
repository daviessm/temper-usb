# temper-usb

Queries a TEMPer USB device to get temperatures.
If no arguments are given, prints the temperature to stdout.

```
USAGE:
    temper-usb [FLAGS] [OPTIONS]

FLAGS:
        --help       Prints help information
    -h, --http       Serve temperatures via HTTP
    -m, --mqtt       Publish readings at regular intervals over MQTT
    -V, --version    Prints version information

OPTIONS:
    -p, --http-port <PORT>              HTTP port to listen on (default 80)
    -f, --mqtt-frequency <FREQUENCY>    Frequency of MQTT readings to send (in seconds, default 30)
    -s, --mqtt-server <SERVER>          IP address or host name, and port number of the MQTT
                                        server, for example localhost:1833
    -t, --mqtt-topic <TOPIC>            MQTT topic onto which to publish readings
```
