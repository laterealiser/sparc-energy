import psycopg2
import uuid
import bcrypt
from datetime import datetime
import random

# Supabase PostgreSQL (IPv4 Transaction Pooler with Project ID prefix)
DATABASE_URL = "postgresql://postgres.loldpnnmjqttgvsxcgnr:PROGRAMMER-CHANDU7%24c@aws-1-ap-south-1.pooler.supabase.com:6543/postgres?sslmode=require"

def seed():
    conn = None
    try:
        conn = psycopg2.connect(DATABASE_URL)
        cur = conn.cursor()
        
        # Check if already seeded (by users)
        cur.execute("SELECT COUNT(*) FROM users")
        count = cur.fetchone()[0]
        
        now = datetime.now().isoformat()
        
        def hash_pw(pw):
            return bcrypt.hashpw(pw.encode('utf-8'), bcrypt.gensalt()).decode('utf-8')

        # 1. Create Users
        users = [
            ("admin@sparcenergy.com", "Admin@123", "Sparc Admin", "admin", 1000000.0),
            ("pdd@certified.com", "Expert@123", "Arjun Malhotra", "pdd_writer", 0.0),
            ("auditor@verra.org", "Expert@123", "Sarah Chen", "auditor", 0.0),
            ("greenforest@reforestation.com", "Seller@123", "Amazon Reforestation Ltd", "seller", 250000.0),
            ("demo@sparcenergy.com", "Demo@123", "Demo Investor", "buyer", 50000.0)
        ]
        
        user_ids = {}
        for email, pw, name, role, bal in users:
            uid = str(uuid.uuid4())
            cur.execute(
                "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s) ON CONFLICT (email) DO NOTHING",
                (uid, email, hash_pw(pw), name, role, bal, 1, now, now)
            )
            cur.execute("SELECT id FROM users WHERE email=%s", (email,))
            user_ids[email] = cur.fetchone()[0]

        # 1b. Create Professional Profiles
        pros = [
            (user_ids["pdd@certified.com"], "Senior PDD Writer", "10+ years in VCS/GS methodology."),
            (user_ids["auditor@verra.org"], "VCS Certified Auditor", "Lead auditor for renewable energy.")
        ]
        for uid, title, bio in pros:
            cur.execute(
                "INSERT INTO professional_profiles (user_id, title, bio, verified, created_at, updated_at) VALUES (%s, %s, %s, 1, %s, %s) ON CONFLICT (user_id) DO NOTHING",
                (uid, title, bio, now, now)
            )

        # 2. Create Projects (If none exist)
        cur.execute("SELECT COUNT(*) FROM carbon_projects")
        proj_count = cur.fetchone()[0]
        
        if proj_count == 0:
            print("Seeding initial projects and institutional matching stack...")
            projects = [
                (str(uuid.uuid4()), "Amazon Reforestation Initiative", "reforestation", "Brazil", "greenforest@reforestation.com", 500000.0, 350000.0, "Verra VCS", "13,15,17", 85000.0, "2020-01-01"),
                (str(uuid.uuid4()), "Rajasthan Solar Farm", "solar", "India", "greenforest@reforestation.com", 200000.0, 180000.0, "Gold Standard", "7,9,13", 42000.0, "2021-06-01")
            ]
            
            project_ids = []
            for pid, name, ptype, country, owner_email, total, issued, cert, sdgs, co2, start in projects:
                owner_id = user_ids[owner_email]
                cur.execute(
                    "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)",
                    (pid, name, f"Sample {ptype} project in {country}", ptype, f"Region in {country}", country, owner_id, total, issued, 1, cert, sdgs, co2, start, now, now)
                )
                project_ids.append((pid, owner_id, cert, name, country, ptype))

            # 3. Create Credits (INR Prices!)
            credits = [
                (str(uuid.uuid4()), project_ids[0][0], project_ids[0][1], 1550.0, 50000.0, 2023, project_ids[0][2], "VCS-BRA-2023-001"),
                (str(uuid.uuid4()), project_ids[1][0], project_ids[1][1], 1840.0, 30000.0, 2024, project_ids[1][2], "GS-IND-2024-001")
            ]
            
            for cid, pid, sid, price, qty, year, cert, serial in credits:
                cur.execute(
                    "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, created_at, updated_at) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)",
                    (cid, pid, sid, price, qty, qty, "active", year, cert, serial, now, now)
                )

            conn.commit()
            print("✅ Database seeded with Production Institutional data!")
        else:
            conn.commit()
            print("✅ Users and Professional Profiles verified.")

    except Exception as e:
        print(f"❌ Error during seeding: {e}")
        if conn: conn.rollback()
    finally:
        if conn:
            cur.close()
            conn.close()

if __name__ == "__main__":
    seed()
