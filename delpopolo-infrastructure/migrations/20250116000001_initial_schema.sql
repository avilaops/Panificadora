-- Create custom types
CREATE TYPE product_category AS ENUM (
    'Bread', 'Cake', 'Cookie', 'Pastry', 'Beverage', 
    'Sandwich', 'Snack', 'RawMaterial', 'Other'
);

CREATE TYPE unit_of_measure AS ENUM (
    'Unit', 'Kilogram', 'Gram', 'Liter', 'Milliliter', 'Dozen', 'Package'
);

CREATE TYPE order_status AS ENUM (
    'Pending', 'Confirmed', 'Preparing', 'Ready', 'InDelivery', 'Completed', 'Cancelled'
);

CREATE TYPE order_source AS ENUM (
    'InStore', 'IFood', 'WhatsApp', 'Web', 'Phone'
);

CREATE TYPE payment_method AS ENUM (
    'Cash', 'DebitCard', 'CreditCard', 'Pix', 'VoucherMeal', 'VoucherFood', 'Multiple'
);

CREATE TYPE payment_status AS ENUM (
    'Pending', 'Processing', 'Approved', 'Rejected', 'Cancelled', 'Refunded'
);

CREATE TYPE movement_type AS ENUM (
    'Purchase', 'Sale', 'Adjustment', 'Loss', 'Return', 'Transfer'
);

CREATE TYPE user_role AS ENUM (
    'Admin', 'Manager', 'Cashier', 'InventoryManager', 'Kitchen', 'Delivery'
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_login_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_is_active ON users(is_active);

-- Suppliers table
CREATE TABLE suppliers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    trade_name VARCHAR(255),
    cnpj VARCHAR(14),
    email VARCHAR(255),
    phone VARCHAR(11),
    whatsapp VARCHAR(11),
    contact_person VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_preferred BOOLEAN NOT NULL DEFAULT false,
    api_url TEXT,
    api_key TEXT,
    excel_catalog_url TEXT,
    excel_last_sync TIMESTAMP WITH TIME ZONE,
    rating REAL,
    total_orders INTEGER NOT NULL DEFAULT 0,
    on_time_delivery_rate REAL,
    payment_terms TEXT,
    min_order_value NUMERIC(10, 2),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_suppliers_is_active ON suppliers(is_active);
CREATE INDEX idx_suppliers_is_preferred ON suppliers(is_preferred);

-- Products table
CREATE TABLE products (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    sku VARCHAR(100) UNIQUE NOT NULL,
    barcode VARCHAR(50),
    category product_category NOT NULL,
    unit_of_measure unit_of_measure NOT NULL,
    price_amount NUMERIC(10, 2) NOT NULL,
    price_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    cost_amount NUMERIC(10, 2) NOT NULL,
    cost_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    stock_quantity NUMERIC(10, 2) NOT NULL DEFAULT 0,
    min_stock_level NUMERIC(10, 2) NOT NULL DEFAULT 10,
    max_stock_level NUMERIC(10, 2),
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_available_online BOOLEAN NOT NULL DEFAULT true,
    image_url TEXT,
    weight NUMERIC(10, 3),
    preparation_time_minutes INTEGER,
    supplier_id UUID REFERENCES suppliers(id),
    nfe_ncm VARCHAR(10),
    nfe_cest VARCHAR(10),
    nfe_cfop VARCHAR(10),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_products_sku ON products(sku);
CREATE INDEX idx_products_barcode ON products(barcode);
CREATE INDEX idx_products_category ON products(category);
CREATE INDEX idx_products_is_active ON products(is_active);
CREATE INDEX idx_products_stock_quantity ON products(stock_quantity);
CREATE INDEX idx_products_supplier_id ON products(supplier_id);

-- Customers table
CREATE TABLE customers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    cpf VARCHAR(11) UNIQUE NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(11),
    birth_date TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    accepts_marketing BOOLEAN NOT NULL DEFAULT false,
    loyalty_points INTEGER NOT NULL DEFAULT 0,
    total_orders INTEGER NOT NULL DEFAULT 0,
    total_spent NUMERIC(10, 2) NOT NULL DEFAULT 0,
    whatsapp_optin BOOLEAN NOT NULL DEFAULT false,
    whatsapp_number VARCHAR(13),
    fcm_token TEXT,
    last_order_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_customers_cpf ON customers(cpf);
CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_is_active ON customers(is_active);
CREATE INDEX idx_customers_whatsapp_optin ON customers(whatsapp_optin);

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    order_number VARCHAR(50) UNIQUE NOT NULL,
    customer_id UUID REFERENCES customers(id),
    customer_name VARCHAR(255),
    customer_cpf VARCHAR(11),
    subtotal_amount NUMERIC(10, 2) NOT NULL,
    subtotal_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    discount_amount NUMERIC(10, 2) NOT NULL DEFAULT 0,
    discount_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    delivery_fee_amount NUMERIC(10, 2) NOT NULL DEFAULT 0,
    delivery_fee_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    total_amount NUMERIC(10, 2) NOT NULL,
    total_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    status order_status NOT NULL,
    source order_source NOT NULL,
    payment_method payment_method,
    payment_id UUID,
    is_paid BOOLEAN NOT NULL DEFAULT false,
    delivery_address TEXT,
    delivery_time TIMESTAMP WITH TIME ZONE,
    ifood_order_id VARCHAR(100),
    ifood_reference VARCHAR(100),
    table_number VARCHAR(20),
    turnstile_entry_id UUID,
    notes TEXT,
    estimated_preparation_time INTEGER,
    preparation_started_at TIMESTAMP WITH TIME ZONE,
    ready_at TIMESTAMP WITH TIME ZONE,
    delivered_at TIMESTAMP WITH TIME ZONE,
    cancelled_at TIMESTAMP WITH TIME ZONE,
    cancellation_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_orders_order_number ON orders(order_number);
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_source ON orders(source);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);

-- Order items table
CREATE TABLE order_items (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    product_name VARCHAR(255) NOT NULL,
    quantity NUMERIC(10, 2) NOT NULL,
    unit_price_amount NUMERIC(10, 2) NOT NULL,
    unit_price_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    total_price_amount NUMERIC(10, 2) NOT NULL,
    total_price_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    notes TEXT
);

CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_product_id ON order_items(product_id);

-- Inventory table
CREATE TABLE inventory (
    id UUID PRIMARY KEY,
    product_id UUID UNIQUE NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity NUMERIC(10, 2) NOT NULL DEFAULT 0,
    reserved_quantity NUMERIC(10, 2) NOT NULL DEFAULT 0,
    available_quantity NUMERIC(10, 2) NOT NULL DEFAULT 0,
    last_movement_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_inventory_product_id ON inventory(product_id);

-- Inventory movements table
CREATE TABLE inventory_movements (
    id UUID PRIMARY KEY,
    product_id UUID NOT NULL REFERENCES products(id),
    movement_type movement_type NOT NULL,
    quantity NUMERIC(10, 2) NOT NULL,
    unit_cost NUMERIC(10, 2),
    total_cost NUMERIC(10, 2),
    order_id UUID REFERENCES orders(id),
    supplier_id UUID REFERENCES suppliers(id),
    nfe_key VARCHAR(44),
    notes TEXT,
    performed_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_inventory_movements_product_id ON inventory_movements(product_id);
CREATE INDEX idx_inventory_movements_movement_type ON inventory_movements(movement_type);
CREATE INDEX idx_inventory_movements_created_at ON inventory_movements(created_at DESC);

-- Payments table
CREATE TABLE payments (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES orders(id),
    customer_id UUID REFERENCES customers(id),
    amount_value NUMERIC(10, 2) NOT NULL,
    amount_currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    payment_method payment_method NOT NULL,
    status payment_status NOT NULL,
    card_last_digits VARCHAR(4),
    card_brand VARCHAR(50),
    authorization_code VARCHAR(50),
    nsu VARCHAR(50),
    pix_key VARCHAR(255),
    pix_qr_code TEXT,
    pix_txid VARCHAR(100),
    pos_transaction_id VARCHAR(100),
    pos_terminal_id VARCHAR(50),
    cash_received NUMERIC(10, 2),
    change_amount NUMERIC(10, 2),
    notes TEXT,
    paid_at TIMESTAMP WITH TIME ZONE,
    cancelled_at TIMESTAMP WITH TIME ZONE,
    refunded_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_payments_customer_id ON payments(customer_id);
CREATE INDEX idx_payments_status ON payments(status);

-- Turnstile entries table
CREATE TABLE turnstile_entries (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES customers(id),
    customer_cpf VARCHAR(11),
    customer_name VARCHAR(255),
    entry_time TIMESTAMP WITH TIME ZONE NOT NULL,
    exit_time TIMESTAMP WITH TIME ZONE,
    table_number VARCHAR(20),
    order_id UUID REFERENCES orders(id),
    total_spent NUMERIC(10, 2) NOT NULL DEFAULT 0,
    is_paid BOOLEAN NOT NULL DEFAULT false,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_turnstile_entries_customer_id ON turnstile_entries(customer_id);
CREATE INDEX idx_turnstile_entries_entry_time ON turnstile_entries(entry_time DESC);
