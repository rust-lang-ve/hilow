#!/bin/bash

systemfd --no-pid -s http::0.0.0.0:7878 -- cargo watch -x run
