import psycopg2
import uuid
import bcrypt
from datetime import datetime
import random

DATABASE_URL = "postgresql://neondb_owner:npg_V28glLuFXozr@ep-gentle-math-am7g1h2t-pooler.c-5.us-east-1.aws.neon.tech/neondb?sslmode=require"

def seed():
    conn = None
    try:
        conn = psycopg2.connect(DATABASE_URL)
        cur = conn.cursor()
        
        # Check if already seeded (by projects)
        cur.execute("SELECT COUNT(*) FROM carbon_projects")
        count = cur.fetchone()[0]
        
        if count > 0:
            print("Database already has projects, skipping...")
            # We still want to make sure users exist for testing
        else:
            print("Seeding database with sample projects and credits...")

        now = datetime.now().isoformat()
        
        def hash_pw(pw):
            return bcrypt.hashpw(pw.encode('utf-8'), bcrypt.gensalt()).decode('utf-8')

        # 1. Create Users (with ON CONFLICT)
        users = [
            ("admin@sparcenergy.com", "Admin@123", "Sparc Admin", "admin", 1000000.0),
            ("greenforest@reforestation.com", "Seller@123", "Amazon Reforestation Ltd", "seller", 250000.0),
            ("solar@renewableindia.com", "Seller@123", "Renewable India Power", "seller", 180000.0),
            ("wind@nordicclean.com", "Seller@123", "Nordic Clean Energy", "seller", 320000.0),
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

        if count == 0:
            # 2. Create Projects
            projects = [
                (str(uuid.uuid4()), "Amazon Reforestation Initiative", "reforestation", "Brazil", "greenforest@reforestation.com", 500000.0, 350000.0, "Verra VCS", "13,15,17", 85000.0, "2020-01-01"),
                (str(uuid.uuid4()), "Rajasthan Solar Farm", "solar", "India", "solar@renewableindia.com", 200000.0, 180000.0, "Gold Standard", "7,9,13", 42000.0, "2021-06-01"),
                (str(uuid.uuid4()), "North Sea Wind Offshore", "wind", "Norway", "wind@nordicclean.com", 300000.0, 280000.0, "Gold Standard", "7,8,13", 65000.0, "2019-03-01"),
                (str(uuid.uuid4()), "Gujarat Mangrove Conservation", "blue_carbon", "India", "solar@renewableindia.com", 150000.0, 90000.0, "Verra VCS", "14,15,13", 28000.0, "2022-01-01")
            ]
            
            project_ids = []
            for pid, name, ptype, country, owner_email, total, issued, cert, sdgs, co2, start in projects:
                owner_id = user_ids[owner_email]
                cur.execute(
                    "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)",
                    (pid, name, f"Sample {ptype} project in {country}", ptype, f"Region in {country}", country, owner_id, total, issued, 1, cert, sdgs, co2, start, now, now)
                )
                project_ids.append((pid, owner_id, cert, name, country, ptype))

            # 3. Create Credits
            credits = [
                (str(uuid.uuid4()), project_ids[0][0], project_ids[0][1], 18.50, 50000.0, 2023, project_ids[0][2], "VCS-BRA-2023-001"),
                (str(uuid.uuid4()), project_ids[1][0], project_ids[1][1], 22.75, 30000.0, 2024, project_ids[1][2], "GS-IND-2024-001"),
                (str(uuid.uuid4()), project_ids[2][0], project_ids[2][1], 31.20, 25000.0, 2024, project_ids[2][2], "GS-NOR-2024-001"),
                (str(uuid.uuid4()), project_ids[3][0], project_ids[3][1], 14.80, 20000.0, 2023, project_ids[3][2], "VCS-IND-2023-002"),
                (str(uuid.uuid4()), project_ids[0][0], project_ids[0][1], 16.40, 40000.0, 2022, project_ids[0][2], "VCS-BRA-2022-001")
            ]
            
            for cid, pid, sid, price, qty, year, cert, serial in credits:
                cur.execute(
                    "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)",
                    (cid, pid, sid, price, qty, qty, "active", year, cert, serial, "VM0007", now, now)
                )
                
                # Price history
                for i in range(30):
                    hp = round(price * (0.85 + random.random() * 0.3), 2)
                    hv = float(random.randint(100, 2000))
                    hid = str(uuid.uuid4())
                    cur.execute(
                        "INSERT INTO price_history (id, credit_id, price, volume, recorded_at) VALUES (%s, %s, %s, %s, %s)",
                        (hid, cid, hp, hv, now)
                    )

            conn.commit()
            print("✅ Database seeded successfully with production data!")
        else:
            conn.commit()
            print("✅ Database users verified, projects already exist.")

    except Exception as e:
        print(f"❌ Error during seeding: {e}")
        if conn: conn.rollback()
    finally:
        if conn:
            cur.close()
            conn.close()

if __name__ == "__main__":
    seed()
