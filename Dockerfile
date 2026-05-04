FROM alpine:3.20

RUN echo "Hello from step 1"
RUN echo "Hello from step 2"

CMD ["echo", "build finished"]