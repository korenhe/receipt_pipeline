# ---------- build stage ----------
FROM nvidia/cuda:13.0.0-runtime-ubuntu22.04 AS build.image
# ^ devel image needed to build Python wheels with CUDA

ARG HTTP_PROXY
ARG HTTPS_PROXY
ARG http_proxy
ARG https_proxy

ENV PIP_NO_CACHE_DIR=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

RUN apt-get update && apt-get install -y \
    python3.10 \
    python3-pip \
    python3-dev \
    build-essential \
    git \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN ln -s /usr/bin/python3 /usr/bin/python

WORKDIR /wheels

# ---- GPU PaddleOCR stack ----
RUN pip install --upgrade pip \
 && pip wheel \
    paddleocr \
    PyMuPDF \
    pillow \
    numpy

RUN pip install paddlepaddle-gpu==3.2.2 -i https://www.paddlepaddle.org.cn/packages/stable/cu130/
# python -m pip install paddlepaddle-gpu==3.2.2 -i https://www.paddlepaddle.org.cn/packages/stable/cu130/

# ---------- runtime / dev stage ----------
FROM nvidia/cuda:13.0.0-runtime-ubuntu22.04 AS dev.image
# ^ runtime image is smaller and sufficient

ARG HTTP_PROXY
ARG HTTPS_PROXY
ARG http_proxy
ARG https_proxy

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH

# ---- system deps (Python + Rust + OCR runtime) ----
RUN apt-get update && apt-get install -y \
    python3.10 \
    python3-pip \
    python3-dev \
    libgomp1 \
    libgl1 \
    libglib2.0-0 \
    libsm6 \
    libxrender1 \
    libxext6 \
    poppler-utils \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN ln -s /usr/bin/python3 /usr/bin/python

# ---- install Rust toolchain ----
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile default \
    && rustup component add rustfmt clippy

# ---- Python wheels ----
WORKDIR /app
COPY --from=build.image /wheels /wheels

# Install Paddle GPU first (not in wheels)
RUN pip install paddlepaddle-gpu==3.2.2 \
    -i https://www.paddlepaddle.org.cn/packages/stable/cu130/

RUN pip install /wheels/*

# ---- preload PaddleOCR Chinese models (GPU) ----
RUN python - <<EOF
from paddleocr import PaddleOCR
ocr = PaddleOCR(lang="ch", use_gpu=True)
print("PaddleOCR GPU ready")
EOF

CMD ["bash"]
