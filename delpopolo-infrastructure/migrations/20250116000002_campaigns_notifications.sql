-- Campaigns table
CREATE TYPE campaign_type AS ENUM (
    'Promotional', 'Seasonal', 'FreshBread', 'Birthday', 
    'Welcome', 'Reactivation', 'ThankYou', 'Newsletter'
);

CREATE TYPE campaign_status AS ENUM (
    'Draft', 'Scheduled', 'Active', 'Paused', 'Completed', 'Cancelled'
);

CREATE TYPE campaign_channel AS ENUM (
    'Email', 'WhatsApp', 'Push', 'SMS', 'IFood'
);

CREATE TABLE campaigns (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    campaign_type campaign_type NOT NULL,
    status campaign_status NOT NULL,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE,
    target_all_customers BOOLEAN NOT NULL DEFAULT true,
    target_vip_only BOOLEAN NOT NULL DEFAULT false,
    target_new_customers BOOLEAN NOT NULL DEFAULT false,
    message_template TEXT NOT NULL,
    image_url TEXT,
    cta_text VARCHAR(100),
    cta_url TEXT,
    total_sent INTEGER NOT NULL DEFAULT 0,
    total_delivered INTEGER NOT NULL DEFAULT 0,
    total_opened INTEGER NOT NULL DEFAULT 0,
    total_clicked INTEGER NOT NULL DEFAULT 0,
    total_conversions INTEGER NOT NULL DEFAULT 0,
    revenue_generated NUMERIC(10, 2) NOT NULL DEFAULT 0,
    is_recurring BOOLEAN NOT NULL DEFAULT false,
    recurrence_pattern VARCHAR(100),
    next_execution TIMESTAMP WITH TIME ZONE,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_campaigns_status ON campaigns(status);
CREATE INDEX idx_campaigns_start_date ON campaigns(start_date);
CREATE INDEX idx_campaigns_campaign_type ON campaigns(campaign_type);

-- Campaign channels junction table
CREATE TABLE campaign_channels (
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    channel campaign_channel NOT NULL,
    PRIMARY KEY (campaign_id, channel)
);

-- Notifications table
CREATE TYPE notification_type AS ENUM (
    'LowStock', 'OrderReceived', 'OrderReady', 'OrderDelivered', 
    'PaymentReceived', 'CampaignMessage', 'SystemAlert'
);

CREATE TYPE notification_channel AS ENUM (
    'Email', 'Push', 'SMS', 'WhatsApp'
);

CREATE TYPE notification_status AS ENUM (
    'Pending', 'Sent', 'Delivered', 'Read', 'Failed'
);

CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    recipient_id UUID REFERENCES customers(id),
    recipient_email VARCHAR(255),
    recipient_phone VARCHAR(13),
    recipient_fcm_token TEXT,
    notification_type notification_type NOT NULL,
    channel notification_channel NOT NULL,
    status notification_status NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    data JSONB,
    sent_at TIMESTAMP WITH TIME ZONE,
    delivered_at TIMESTAMP WITH TIME ZONE,
    read_at TIMESTAMP WITH TIME ZONE,
    failed_at TIMESTAMP WITH TIME ZONE,
    failure_reason TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_notifications_recipient_id ON notifications(recipient_id);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);

-- Supplier products catalog
CREATE TABLE supplier_products (
    id UUID PRIMARY KEY,
    supplier_id UUID NOT NULL REFERENCES suppliers(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    supplier_sku VARCHAR(100) NOT NULL,
    unit_price NUMERIC(10, 2) NOT NULL,
    min_order_quantity NUMERIC(10, 2),
    lead_time_days INTEGER,
    is_available BOOLEAN NOT NULL DEFAULT true,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE(supplier_id, product_id)
);

CREATE INDEX idx_supplier_products_supplier_id ON supplier_products(supplier_id);
CREATE INDEX idx_supplier_products_product_id ON supplier_products(product_id);
CREATE INDEX idx_supplier_products_is_available ON supplier_products(is_available);
